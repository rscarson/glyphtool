//! Module dealing with raw-text processing and tokenization
use crate::error::EtroisResult;
use phonambulator::PhonambulationSource;

pub mod collections;
pub mod phonambulator;
pub mod preprocessor;

mod token;
mod tokens_ext;
pub use token::Token;
pub use tokens_ext::TokensExt;

/// Parse a string into a list of tokens
#[must_use]
pub fn lex(text: &str) -> Vec<Token> {
    Vec::from_string(text)
}

/// Parse a string of text into a structured format
///
/// # Arguments
/// - `text` - The text to parse
/// - `db_path` - The path to the database
/// - `phonambulation_src` - The source for user phoneme inputs
/// - `skip_translation` - Skip the translation step, for writing in native E'trois
///
/// # Errors
/// Will return an error if the phonambulation fails
pub fn parse<S: PhonambulationSource>(
    text: &str,
    db_path: Option<&str>,
    phonambulation_src: S,
    skip_translation: bool,
) -> EtroisResult<collections::Text> {
    println!("Preprocessing...");
    let text = preprocessor::preprocess_text(text);

    println!("Lexing...");
    let mut tokens = lex(&text);

    if !skip_translation {
        println!("Phonambulating...");
        tokens.phonambulate(db_path, phonambulation_src)?;
    }

    println!("Reducing...");
    tokens.reduce();

    println!("Collecting...");
    Ok(collections::Text::from_tokens(&tokens))
}
