//! The part of the translation process that converts a word into a set of phonemes.
//!
//! The word will be split into groups by ', and each group is looked up in the database
//!
//! If nothing is found, the user will be prompted to enter a new phonambulation.
//!
use super::collections::WordKind;
use crate::{
    database::Database,
    error::{Error, EtroisResult},
    glyphs::AsGlyphs,
};

mod ipa;
mod syllabic_model;

pub use ipa::PhonemeExt;
pub use syllabic_model::SyllabicModel;

/// Trait for providing user input for the phonambulator
pub trait PhonambulationSource {
    /// Get the phoneme mapping for an unrecognized word
    ///
    /// Will be called in a loop until a valid phoneme string is returned
    ///
    /// If the input is empty, the suggestion will be used and the db will not be updated
    ///
    /// # Arguments
    /// - `input` - The unrecognized word
    /// - `suggestion` - The automatically generated suggestion
    /// - `error` - The last input with invalid phonemes highlighted (if any)
    ///
    /// # Errors
    /// Can return an error to indicate an unrecoverable fault
    fn get_next(
        &mut self,
        input: &str,
        suggestion: &str,
        error: Option<&str>,
    ) -> EtroisResult<String>;
}

/// The function takes in:
/// - The input word
/// - An automatically generated suggestion
/// - The last input with invalid phonemes highlighted (if any)
pub struct Phonambulator<S: PhonambulationSource> {
    source: S,
    syllables: SyllabicModel,
    db: Database,
}
impl<S: PhonambulationSource> Phonambulator<S> {
    /// Create a new phonambulator
    ///
    /// # Arguments
    /// - `path` - The path to the database
    /// - `source` - The source for phoneme suggestions
    ///
    /// # Errors
    /// Will return an error if the database or syllabic model failed to load
    pub fn new(path: Option<&str>, source: S) -> EtroisResult<Self> {
        let db = Database::new(path)?;
        let syllables = SyllabicModel::new(&db)?;
        Ok(Self {
            source,
            syllables,
            db,
        })
    }

    fn suggestion(&mut self, word: &str) -> EtroisResult<String> {
        let phonemes = word
            .to_glyphs()
            .ok_or(Error::Other("Could not convert to IPA".to_string()))?;
        self.syllables.syllabify(&phonemes)
    }

    fn get_next(&mut self, group: &str) -> EtroisResult<String> {
        let suggestion = self.suggestion(group)?;
        let next = self.source.get_next(group, &suggestion, None)?;
        let next = next.trim();
        if next.is_empty() {
            Ok(suggestion)
        } else {
            let mut next = next.to_string();
            loop {
                match validate_phonemes(&next) {
                    Ok(_) => break,
                    Err(output) => {
                        next = self.source.get_next(group, &suggestion, Some(&output))?;
                    }
                }
            }

            self.db.insert(group, &next)?;
            Ok(next)
        }
    }

    /// Convert a word into a set of phonemes
    ///
    /// The word will be split into groups by ', and each group is looked up in the database  
    /// User input will be requested for unrecognized words
    ///
    /// # Errors
    /// Will return an error if the phoneme source fails, or if the database fails
    pub fn phonambulate(&mut self, word: &str) -> EtroisResult<String> {
        let mut phonemes = vec![];

        for group in word.split('\'') {
            let phoneme = match self.db.get_encoded(group)? {
                Some(phoneme) => phoneme,
                None => self.get_next(group)?,
            };

            phonemes.push(phoneme);
        }

        let phonemes = phonemes.join("'").replace("''", "'");
        Ok(phonemes)
    }
}

/// Validate that user provided phonemes are valid
///
/// # Errors
/// If valid, a trimmed version of the input is returned  
/// Otherwise, the input is returned with invalid phonemes replaced with '?'
pub fn validate_phonemes(phonemes: &str) -> Result<&str, String> {
    let phonemes = phonemes.trim();
    let groups = phonemes.split('-').map(ToString::to_string).collect();
    let glyphs = WordKind::PhonemeGroup(groups);
    let glyphs = glyphs.as_glyphs();
    let glyphs = glyphs.into_iter().flatten();
    let glyphs: String = glyphs.map(|g| g.pronounciation()).collect();
    if glyphs.contains('?') {
        Err(glyphs)
    } else {
        Ok(phonemes)
    }
}

/// A phonambulation source that always accepts the automatically generated suggestion
pub struct AlwaysAutoSource;
impl PhonambulationSource for AlwaysAutoSource {
    fn get_next(&mut self, _: &str, _: &str, _: Option<&str>) -> EtroisResult<String> {
        Ok(String::new())
    }
}
