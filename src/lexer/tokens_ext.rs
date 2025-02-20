use std::mem::take;

use logos::Logos;

use super::{phonambulator::Phonambulator, preprocessor::preprocess_word, token::Token};
use crate::error::{Error, Result};

pub trait TokensExt {
    /// Parse a string into a set of tokens
    fn from_string(text: &str) -> Result<Self>
    where
        Self: Sized;

    /// Goes through all word tokens, replacing them with the stored phonetic representation.
    ///
    /// If none is found, user will be prompted to enter a new phonetic string
    ///
    /// For example magic -> mad-shik
    fn phonambulate(&mut self, db_path: Option<&str>, auto: bool) -> Result<()>;

    /// Reduce the tokenstream down, forming compound words, and removing extraneous boundaries
    ///
    /// Modifies the tokenstream in place. Perform phonambulation before calling this method.
    fn reduce(&mut self);
}
impl TokensExt for Vec<Token> {
    fn from_string(input: &str) -> Result<Self> {
        // We go to lowercase - being careful to leave E' O' and A' as-is
        let mut text = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if ['A', 'E', 'O'].contains(&c) {
                if let Some('\'') = chars.peek() {
                    text.push(c);
                    continue;
                }
            }

            text.push(c.to_ascii_lowercase());
        }

        let mut tokens = vec![];
        let mut lexer = Token::lexer(&text);
        while let Some(token) = lexer.next() {
            match token {
                Ok(token) => tokens.push(token),
                Err(_) => Err(Error::Lexer(lexer.slice().to_string()))?,
            }
        }

        Ok(tokens)
    }

    fn phonambulate(&mut self, db_path: Option<&str>, auto: bool) -> Result<()> {
        let mut phonambulator = Phonambulator::new(db_path, auto)?;
        for token in self {
            if let Token::Word(word) = token {
                let phonemes = phonambulator.phonambulate(word)?;
                *word = phonemes;
            }
        }

        Ok(())
    }

    fn reduce(&mut self) {
        let mut tokens = Vec::with_capacity(self.len());
        let mut compound_word = String::new();
        for token in self.drain(..) {
            match token {
                //
                // Numeric value
                Token::Number(n) => tokens.push(Token::Number(n)),

                //
                // Continuation of the current word
                Token::Word(word) => {
                    let word = preprocess_word(&word);

                    if !compound_word.is_empty() {
                        compound_word.push('\'');
                    }
                    compound_word.push_str(&word);
                }

                //
                // End of the current word
                Token::WordBoundary => {
                    if !compound_word.is_empty() {
                        let word = take(&mut compound_word);
                        tokens.push(Token::Word(word));
                    }
                }

                //
                // The word is prefixed with the deific mark (E')
                Token::DeificModifier => {
                    compound_word.insert(0, '\'');
                    compound_word.insert(0, 'E');
                }

                //
                // The word is prefixed with the ownership mark (O')
                Token::OwnershipModifier => {
                    compound_word.insert(0, '\'');
                    compound_word.insert(0, 'O');
                }

                //
                // End of a sentence, and also therefore the end of the current word
                Token::SentenceBoundary => {
                    if !compound_word.is_empty() {
                        let word = take(&mut compound_word);
                        tokens.push(Token::Word(word));
                    }
                    tokens.push(Token::SentenceBoundary);
                }

                //
                // End of a line, and also therefore the end of the current word
                Token::LineBoundary => {
                    if !compound_word.is_empty() {
                        let word = take(&mut compound_word);
                        tokens.push(Token::Word(word));
                    }
                    tokens.push(Token::LineBoundary);
                }
            }
        }

        //
        // Push the last word, if any
        if !compound_word.is_empty() {
            let word = take(&mut compound_word);
            tokens.push(Token::Word(word));
        }

        //
        // Remove trailing and leading Line/Sentence boundaries
        while let Some(Token::LineBoundary | Token::SentenceBoundary) = tokens.first() {
            tokens.remove(0);
        }
        while let Some(Token::LineBoundary | Token::SentenceBoundary) = tokens.last() {
            tokens.pop();
        }

        *self = tokens;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce() {
        let text = "sword of the gods. word of god. son of mark. 2nd potato.";
        let mut tokens = Vec::<Token>::from_string(text).unwrap();
        tokens.reduce();

        assert_eq!(
            tokens,
            vec![
                Token::Word("E'sword".to_string()),
                Token::SentenceBoundary,
                Token::Word("E'word".to_string()),
                Token::SentenceBoundary,
                Token::Word("O'son'mark".to_string()),
                Token::SentenceBoundary,
                Token::Number(2),
                Token::Word("potato".to_string()),
            ]
        );
    }
}
