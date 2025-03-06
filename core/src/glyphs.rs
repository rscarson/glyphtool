//! The language's phonetic glyphs
//! Stores definitions for all the phonetic glyphs in the language
//! and the encoding table for the language, used to map phonetic ascii strings to glyphs
use crate::{lexer::collections::WordKind, renderer::render_trait::Renderer};

/// Represents a single phonetic glyph in the language
pub trait Glyph: Renderer {
    /// Returns the ascii pronounciation string for this glyph
    fn pronounciation(&self) -> &'static str;

    /// Returns an owned boxed version of the glyph
    fn as_boxed(&self) -> Box<dyn Glyph>;
}

macro_rules! glyph {
    ($name:ident => $pronounciation:literal, $($docs:literal)+) => {
        #[derive(Clone, Copy)]
        $( #[doc = $docs] )+
        pub struct $name;
        impl Glyph for $name {
            fn pronounciation(&self) -> &'static str {
                $pronounciation
            }

            fn as_boxed(&self) -> Box<dyn Glyph> {
                Box::new($name)
            }
        }
    };
}

pub mod consonants;
pub mod numeric;
pub mod special;
pub mod vowels;

mod encoder;
pub use encoder::{AsGlyphs, EncodingTable, ENCODING_TABLE};
