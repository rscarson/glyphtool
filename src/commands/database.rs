use crate::{database::Database, error::Result, lexer::phonambulator::validate_phonemes};
use clap::Subcommand;

#[derive(Debug, clap::Parser)]
pub struct Db {
    #[clap(subcommand)]
    #[command(verbatim_doc_comment)]
    commands: Commands,

    /// Optional path to a database file. If not provided `phonemes.db` is used
    #[arg(short, long)]
    path: Option<String>,
}
impl Db {
    pub fn exec(&self) -> Result<()> {
        self.commands.exec(self.path.as_deref())
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a word and its translation to the database
    Add {
        /// The untranslated word to add (case-insensitive)
        word: String,

        /// The phonemes to map to the word (for example, `mad-shik` for `magic`)
        phonemes: String,
    },

    /// Add a set of words and their phonemes to the database (one per line, whitespace separated)
    AddAll {
        /// The file to read words and phonemes from
        path: String,
    },

    /// Delete a word and its phonemes from the database
    Del {
        /// The untranslated word to delete (case-insensitive)
        word: String,
    },

    /// List all words in the database, optionally filtering by phonemes present
    List {
        /// Optional search term to filter words by phonemes
        search: Option<String>,
    },
}
impl Commands {
    pub fn exec(&self, path: Option<&str>) -> Result<()> {
        let db = Database::new(path)?;
        let path = db.path();
        match self {
            Self::Add { word, phonemes } => match validate_phonemes(phonemes) {
                Ok(phonemes) => {
                    db.insert(word, phonemes)?;
                    println!("Added 1 word to {path}");
                }

                Err(e) => {
                    eprintln!("Unrecognized phoneme in `{e}`");
                }
            },

            Self::AddAll { path } => {
                let file = std::fs::read_to_string(path)?;
                let mut count = 0;
                for line in file.lines() {
                    let mut parts = line.split_whitespace();

                    let Some(word) = parts.next() else {
                        continue;
                    };

                    let Some(phonemes) = parts.next() else {
                        continue;
                    };

                    db.insert(word, phonemes)?;
                    count += 1;
                }

                println!("Added {count} words to {path}");
            }

            Self::Del { word } => {
                db.delete(word)?;
                println!("Deleted 1 word from {path}");
            }

            Self::List { search } => {
                let search = search.as_ref();
                let search = search.map(|s| db.search(s)).unwrap_or_else(|| db.all())?;
                let max_len = search.iter().map(|(w, _)| w.len()).max().unwrap_or(0);
                for (word, phonemes) in search {
                    println!("{word: <max_len$} {phonemes}");
                }
            }
        }

        Ok(())
    }
}
