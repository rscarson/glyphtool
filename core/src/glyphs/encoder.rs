use super::*;
use std::borrow::Cow;

/// A structure defining numeric codes for each glyph
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodingTable {
    inner: Vec<Cow<'static, str>>,
}
impl EncodingTable {
    /// Create a new encoding table from the data in this version of libglyphtool
    pub fn new() -> Self {
        let mut inner = ENCODING_TABLE
            .iter()
            .map(|g| Cow::Borrowed(g.pronounciation()))
            .collect::<Vec<_>>();
        inner.push(Cow::Borrowed("-"));
        Self { inner }
    }

    /// Create a new encoding table from the given data
    #[must_use]
    pub fn from_data(mut data: Vec<(u8, String)>) -> Self {
        data.sort_by(|(a, _), (b, _)| a.cmp(b));
        let inner = data.into_iter().map(|(_, s)| Cow::Owned(s)).collect();
        Self { inner }
    }

    /// Get the length of the encoding table
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the encoding table is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Decode a series of bytes into a string of phonemes
    #[must_use]
    pub fn decode_word(&self, word: &[u8]) -> String {
        word.iter().map(|phoneme| self.decode(*phoneme)).collect()
    }

    /// Encode a string of phonemes into a series of bytes
    #[must_use]
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
    #[must_use]
    pub fn encode(&self, phoneme: &str) -> u8 {
        self.inner
            .iter()
            .position(|s| s == phoneme)
            .unwrap_or_default() as u8
    }

    /// Decode an index into a phoneme
    #[must_use]
    pub fn decode(&self, index: u8) -> &str {
        self.inner.get(index as usize).map_or("?", |s| s.as_ref())
    }

    /// Get the mappings for this encoding table
    #[must_use]
    pub fn mappings(&self) -> &[Cow<'static, str>] {
        &self.inner
    }
}

impl Default for EncodingTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Used to convert a word into a series of glyphs
pub trait AsGlyphs {
    /// Split a word into syllable stacks, each containing a series of glyphs
    fn as_glyphs(&self) -> Vec<Vec<Box<dyn Glyph>>>;
}
impl AsGlyphs for WordKind {
    fn as_glyphs(&self) -> Vec<Vec<Box<dyn Glyph>>> {
        match self {
            //
            // If the token is a number, return the numeric glyph
            WordKind::Number(n) => vec![vec![numeric::Number::new(u32::from(*n)).as_boxed()]],

            //
            // If the token is a phoneme group, decode each syllable
            WordKind::PhonemeGroup(groups) => {
                let mut stacks = vec![];
                for group in groups {
                    let mut phonemes = group.as_str();
                    let mut glyphs = vec![];

                    while !phonemes.is_empty() {
                        if phonemes.starts_with('|') {
                            phonemes = &phonemes[1..];

                            // End the current vowel stack
                            if !glyphs.is_empty() {
                                stacks.push(glyphs);
                                glyphs = vec![];
                            }

                            // Add a vowel stack containing just the cartouche glyph
                            stacks.push(vec![special::Cartouche.as_boxed()]);

                            continue;
                        }

                        if phonemes.starts_with('\'') {
                            phonemes = &phonemes[1..];

                            // End the current vowel stack
                            if !glyphs.is_empty() {
                                stacks.push(glyphs);
                                glyphs = vec![];
                            }
                            continue;
                        }

                        let mut has_match = false;
                        for glyph in ENCODING_TABLE {
                            let sound = glyph.pronounciation();

                            if phonemes.starts_with(sound) {
                                if STOP_SOUNDS.contains(&sound) {
                                    // End the current vowel stack
                                    if !glyphs.is_empty() {
                                        stacks.push(glyphs);
                                        glyphs = vec![];
                                    }

                                    stacks.push(vec![glyph.as_boxed()]);
                                } else {
                                    glyphs.push(glyph.as_boxed());
                                }

                                phonemes = &phonemes[sound.len()..];
                                has_match = true;
                                break;
                            }
                        }

                        if !has_match {
                            // If we reach this point, we have an unrecognized phoneme
                            glyphs.push(special::Unknown.as_boxed());
                            phonemes = &phonemes[1..];
                        }
                    }

                    if !glyphs.is_empty() {
                        stacks.push(glyphs);
                    }
                }

                stacks
            }
        }
    }
}

/// Sounds that stop the current syllable
/// Does not include the midword stop '
pub const STOP_SOUNDS: &[&str] = &[
    ":", // Word boundary
    ".", // Sentence boundary
    "E'", "A'", "O'", // Special stops
];

/// The raw encoding table for the language
/// Represents all the standard glyphs, and the search order for decoding
pub const ENCODING_TABLE: &[&dyn Glyph] = &[
    //
    // Special characters
    &special::Unknown,
    &special::Spacer,
    &special::WordStop,
    &special::SentenceStop,
    &special::Cartouche,
    &special::Deific,
    &special::Posessive,
    &special::Honourific,
    //
    // Closed Consonants
    &consonants::M,
    &consonants::F,
    &consonants::B,
    &consonants::P,
    //
    // Open Consonants
    &consonants::SH,
    &consonants::TH,
    &consonants::NG,
    &consonants::R,
    &consonants::S,
    &consonants::T,
    &consonants::L,
    &consonants::N,
    &consonants::S,
    &consonants::Z,
    &consonants::K,
    &consonants::D,
    //
    // Vowels
    &vowels::AH,
    &vowels::UH,
    &vowels::A,
    &vowels::U,
    &vowels::I,
    &vowels::E,
    &vowels::O,
];
