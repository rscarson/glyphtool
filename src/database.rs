//! A manager for an SQLite database mapping words to phoneme groups
//!
//! For example, magic -> mad-shik
//!
//! It is effectively just a giant database keyed from one string, to another string
//!
use std::{borrow::Cow, collections::HashMap};

use crate::{error::Result, renderer::glyphs};
use rusqlite::{params, Connection};

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
const DB_PATH: &str = "phonemes.db";

pub struct Database {
    path: String,
    conn: Connection,
    encoder: EncodingTable,
}
impl Database {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let path = path.unwrap_or(DB_PATH);
        let conn = Connection::open(path)?;

        conn.execute(TABLE_ENCODING, [])?;
        conn.execute(TABLE_WORDS, [])?;

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
        let path = path.to_string();
        let mut inst = Self {
            conn,
            encoder,
            path,
        };
        let mut mappings = inst.all_mappings()?;

        // Replace the encoding table
        let encoder = EncodingTable::new();
        inst.conn.execute("DELETE FROM encoding", [])?;
        for (i, phoneme) in encoder.mappings().iter().enumerate() {
            inst.conn.execute(
                "INSERT INTO encoding (id, phoneme) VALUES (?1, ?2)",
                params![i, phoneme],
            )?;
        }

        // Reencode the mappings to the current encoding data
        if inst.encoder.reencode(&inst.encoder, &mut mappings) {
            // Remap the data
            inst.conn.execute("DELETE FROM words", [])?;
            for (word, phonemes) in mappings {
                inst.insert_mapping(&word, &phonemes)?;
            }
        }

        inst.encoder = encoder;
        Ok(inst)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn insert_mapping(&self, word: &str, phonemes: &[u8]) -> Result<()> {
        // Insert the word into the database
        self.conn.execute(
            "INSERT INTO words (word, phonemes) VALUES (?1, ?2) ON CONFLICT(word) DO UPDATE SET phonemes = ?2",
            params![word, Some(phonemes)],
        )?;

        Ok(())
    }

    pub fn insert(&self, word: &str, phonemes: &str) -> Result<()> {
        let bytes = self.encoder.encode_word(phonemes);
        self.insert_mapping(word, &bytes)
    }

    pub fn get(&self, word: &str) -> Result<Option<Vec<u8>>> {
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

    pub fn get_encoded(&self, word: &str) -> Result<Option<String>> {
        let map = match self.get(word)? {
            Some(map) => map,
            None => return Ok(None),
        };

        Ok(Some(self.encoder.decode_word(&map)))
    }

    pub fn search(&self, word: &str) -> Result<Vec<(String, String)>> {
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

    pub fn delete(&self, word: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM words WHERE word = ?1", params![word])?;
        Ok(())
    }

    pub fn all_mappings(&self) -> Result<HashMap<String, Vec<u8>>> {
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

    pub fn all(&self) -> Result<Vec<(String, String)>> {
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

#[derive(Debug, PartialEq, Eq)]
pub struct EncodingTable {
    inner: Vec<Cow<'static, str>>,
}
impl EncodingTable {
    pub fn new() -> Self {
        let mut inner = glyphs::GLYPH_SOUND_MAP
            .iter()
            .map(|(s, _)| Cow::Borrowed(*s))
            .collect::<Vec<_>>();
        inner.push(Cow::Borrowed("-"));
        Self { inner }
    }

    pub fn from_data(mut data: Vec<(u8, String)>) -> Self {
        data.sort_by(|(a, _), (b, _)| a.cmp(b));
        let inner = data.into_iter().map(|(_, s)| Cow::Owned(s)).collect();
        Self { inner }
    }

    pub fn decode_word(&self, word: &[u8]) -> String {
        word.iter().map(|phoneme| self.decode(*phoneme)).collect()
    }

    pub fn encode_word(&self, word: &str) -> Vec<u8> {
        let mut indices = vec![];
        let mut buffer = word;
        while !buffer.is_empty() {
            let mut found = false;
            for (i, phoneme) in self.inner.iter().enumerate() {
                if buffer.starts_with(phoneme.as_ref()) {
                    indices.push(i as u8);
                    buffer = &buffer[phoneme.len()..];
                    found = true;
                    break;
                }
            }

            if !found {
                indices.push(0);
                buffer = &buffer[1..];
            }
        }

        indices
    }

    /// Encode a phoneme into an index
    pub fn encode(&self, phoneme: &str) -> u8 {
        self.inner
            .iter()
            .position(|s| s == phoneme)
            .unwrap_or_default() as u8
    }

    /// Decode an index into a phoneme
    pub fn decode(&self, index: u8) -> &str {
        self.inner
            .get(index as usize)
            .map(|s| s.as_ref())
            .unwrap_or("?")
    }

    /// Reencode a mapping from one encoding to another
    pub fn reencode(&self, other: &Self, mappings: &mut HashMap<String, Vec<u8>>) -> bool {
        let mut has_changed = false;
        for word in mappings.values_mut() {
            for index in word.iter_mut() {
                let new = other.encode(self.decode(*index));
                if *index != new {
                    *index = new;
                    has_changed = true;
                }
            }
        }

        has_changed
    }

    /// Get the mappings for this encoding table
    pub fn mappings(&self) -> &[Cow<'static, str>] {
        &self.inner
    }
}
