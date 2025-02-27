//! A manager for an sqlite database mapping words to phoneme groups
//!
//! For example, magic -> mad-shik
//!
//! It is effectively just a giant database keyed from one string, to another string
//!
use crate::{
    error::{Error, EtroisResult},
    glyphs::EncodingTable,
};
use rusqlite::{params, Connection};
use std::collections::HashMap;

//
// Table defs
const TABLE_ENCODING: &str = "CREATE TABLE IF NOT EXISTS encoding (
    id INTEGER PRIMARY KEY,
    phoneme TEXT NOT NULL
)";
const TABLE_WORDS: &str = "CREATE TABLE IF NOT EXISTS words (
    word TEXT UNIQUE PRIMARY KEY,
    phonemes BLOB
);";
const TABLE_MODEL: &str = "CREATE TABLE IF NOT EXISTS model (
    version INTEGER PRIMARY KEY,
    data BLOB,
    corpus_length INTEGER
);";
const DB_PATH: &str = "phonemes.db";

/// A manager for an sqlite database mapping words to phoneme groups
///
/// Contains:
/// - The encoding table
/// - The registered word mappings
/// - The syllabic split model (if trained)
pub struct Database {
    conn: Connection,
    encoder: EncodingTable,
}
impl Database {
    /// Create a new database manager, loading the database at the given path if it exists
    ///
    /// If the database does not exist, it will be created
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn new(path: Option<&str>) -> EtroisResult<Self> {
        let path = path.unwrap_or(DB_PATH);
        let conn = Connection::open(path)?;
        Self::from_connection(conn)
    }

    /// Serialize the database to a byte array
    ///
    /// # Errors
    /// Will return an error if serialization fails
    pub fn serialize(&self) -> EtroisResult<Vec<u8>> {
        let data = self.conn.serialize(rusqlite::DatabaseName::Main)?;
        Ok(data.to_owned())
    }

    /// Loads the database from the given data, resulting in a read-only in-memory database
    ///
    /// # Errors
    /// Will return an error if a database operation fails  
    /// Or if the data is not a valid sqlite database
    pub fn from_serialized(data: &[u8]) -> EtroisResult<Self> {
        let sz = data.len();
        let ptr = std::ptr::NonNull::from(&data[0]);

        let mut conn = Connection::open_in_memory()?;
        let data = unsafe { rusqlite::serialize::OwnedData::from_raw_nonnull(ptr, sz) };

        conn.deserialize(rusqlite::DatabaseName::Main, data, false)?;
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> EtroisResult<Self> {
        conn.execute(TABLE_ENCODING, [])?;
        conn.execute(TABLE_WORDS, [])?;
        conn.execute(TABLE_MODEL, [])?;

        // Load existing encoding data
        let encoder = {
            let mut stmt = conn.prepare("SELECT id, phoneme FROM encoding")?;
            let mut encodings = vec![];
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                let id: u8 = row.get(0)?;
                let phoneme: String = row.get(1)?;
                encodings.push((id, phoneme));
            }

            EncodingTable::from_data(encodings)
        };

        // Load the existing table
        let mut inst = Self { conn, encoder };

        // Get the current encoding data
        let encoder2 = EncodingTable::new();
        if inst.encoder != encoder2 {
            // Extract all the phonemes from the mappings, decoded with the old encoding
            let data = inst.all()?;

            // Reencode the mappings to the current encoding data
            let data = data
                .into_iter()
                .map(|(word, phonemes)| {
                    let phonemes = encoder2.encode_word(&phonemes);
                    (word, phonemes)
                })
                .collect::<Vec<_>>();

            // Remap the data
            for (word, phonemes) in data {
                inst.insert_mapping(&word, &phonemes)?;
            }

            // Warn that the syllabic model is out of date
            eprintln!("Warning: The syllabic model is out of date and must be retrained");

            // Update the encoding table
            inst.encoder = encoder2;
            inst.conn.execute("DELETE FROM encoding", [])?;
            for (i, phoneme) in inst.encoder.mappings().iter().enumerate() {
                inst.conn.execute(
                    "INSERT INTO encoding (id, phoneme) VALUES (?1, ?2)",
                    params![i, phoneme],
                )?;
            }
        }

        Ok(inst)
    }

    /// Get the encoding table
    pub fn encoder(&self) -> &EncodingTable {
        &self.encoder
    }

    /// Save a new syllabic model to the database
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn save_model(&self, data: &[u8]) -> EtroisResult<()> {
        let corpus_length = self.all()?.len();
        self.conn.execute(
            "INSERT INTO model (data, corpus_length) VALUES (?1, ?2)",
            params![data, corpus_length],
        )?;

        // If there are more than 3 models, delete the oldest one
        // This is to prevent the database from growing too large
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM model")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        if count > 3 {
            self.conn
                .execute("DELETE FROM model ORDER BY version ASC LIMIT 1", [])?;
        }

        Ok(())
    }

    /// Load the latest syllabic model from the database
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn load_model(&self) -> EtroisResult<Vec<u8>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM model ORDER BY version DESC LIMIT 1")?;
        let mut rows = stmt.query([])?;
        let data: Vec<u8> = match rows.next()? {
            Some(row) => row.get(0)?,
            None => return Err(Error::Other("No training data found for model".to_string())),
        };

        Ok(data)
    }

    /// Revert the latest syllabic model to the previous version
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn revert_model(&self) -> EtroisResult<()> {
        // Delete the latest model
        self.conn
            .execute("DELETE FROM model ORDER BY version DESC LIMIT 1", [])?;
        Ok(())
    }

    fn insert_mapping(&self, word: &str, phonemes: &[u8]) -> EtroisResult<()> {
        // Insert the word into the database
        self.conn.execute(
            "INSERT INTO words (word, phonemes) VALUES (?1, ?2) ON CONFLICT(word) DO UPDATE SET phonemes = ?2",
            params![word, Some(phonemes)],
        )?;

        Ok(())
    }

    /// Insert a new word mapping into the database
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn insert(&self, word: &str, phonemes: &str) -> EtroisResult<()> {
        let bytes = self.encoder.encode_word(phonemes);
        self.insert_mapping(word, &bytes)
    }

    /// Get the phoneme mapping for a word, as encoded bytes
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn get(&self, word: &str) -> EtroisResult<Option<Vec<u8>>> {
        // Get word ID
        let mut stmt = self
            .conn
            .prepare("SELECT phonemes FROM words WHERE word = ?1")?;
        let mut rows = stmt.query(params![word])?;
        let phonemes: Vec<u8> = match rows.next()? {
            Some(row) => row.get(0)?,
            None => return Ok(None),
        };

        Ok(Some(phonemes))
    }

    /// Get the phoneme mapping for a word, as a string
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn get_encoded(&self, word: &str) -> EtroisResult<Option<String>> {
        let Some(map) = self.get(word)? else {
            return Ok(None);
        };

        Ok(Some(self.encoder.decode_word(&map)))
    }

    /// Search for words that match a given pattern
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn search(&self, word: &str) -> EtroisResult<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT word, phonemes FROM words WHERE word LIKE ?1")?;
        let mut rows = stmt.query(params![word])?;
        let mut words = vec![];
        while let Some(row) = rows.next()? {
            let word: String = row.get(0)?;
            let phonemes: Vec<u8> = row.get(1)?;
            let phonemes = self.encoder.decode_word(&phonemes);
            words.push((word, phonemes));
        }

        Ok(words)
    }

    /// Delete a word mapping from the database
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn delete(&self, word: &str) -> EtroisResult<()> {
        self.conn
            .execute("DELETE FROM words WHERE word = ?1", params![word])?;
        Ok(())
    }

    /// Get all word mappings in the database, in encoded byte form
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn all_mappings(&self) -> EtroisResult<HashMap<String, Vec<u8>>> {
        let mut map = HashMap::new();

        let mut stmt = self.conn.prepare("SELECT word, phonemes FROM words")?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let word: String = row.get(0)?;
            let phonemes: Vec<u8> = row.get(1)?;

            map.insert(word, phonemes);
        }

        Ok(map)
    }

    /// Get all word mappings in the database, in string form
    ///
    /// # Errors
    /// Will return an error if a database operation fails
    pub fn all(&self) -> EtroisResult<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT word, phonemes FROM words")?;
        let mut rows = stmt.query([])?;
        let mut words = vec![];
        while let Some(row) = rows.next()? {
            let word: String = row.get(0)?;
            let phonemes: Vec<u8> = row.get(1)?;
            let phonemes = self.encoder.decode_word(&phonemes);
            words.push((word, phonemes));
        }

        Ok(words)
    }
}
