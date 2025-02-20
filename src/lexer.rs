use token::Token;
use tokens_ext::TokensExt;

pub mod collections;
pub mod phonambulator;
pub mod preprocessor;
pub mod token;
pub mod tokens_ext;

/// Parse a string into a list of tokens
pub fn lex(text: &str) -> crate::error::Result<Vec<Token>> {
    Vec::from_string(text)
}

pub fn parse(
    text: &str,
    db_path: Option<&str>,
    autophonambulate: bool,
) -> crate::error::Result<collections::Text> {
    let text = preprocessor::preprocess_text(text);
    let mut tokens = lex(&text)?;

    tokens.phonambulate(db_path, autophonambulate)?;
    tokens.reduce();
    Ok(collections::Text::from_tokens(&tokens))
}
