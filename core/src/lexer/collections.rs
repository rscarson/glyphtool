//! Contains the data structures that represent the output of the lexer
use super::token::Token;

/// The types of printable tokens that can be found in a sentence
#[derive(Debug, PartialEq)]
pub enum WordKind {
    /// A group of syllables that make up a word
    PhonemeGroup(Vec<String>),

    /// A number
    Number(u16),
}

/// Represents a set of related words in a sentence
#[derive(Debug, PartialEq)]
pub struct Sentence(Vec<WordKind>);
impl Sentence {
    /// Build a sentence from a slice of tokens (presumably bounded by a sentence or line boundary)
    pub fn from_tokens(tokens: &[Token]) -> Self {
        let mut words = vec![];
        for token in tokens {
            match token {
                Token::Number(n) => words.push(WordKind::Number(*n)),
                Token::Word(word) => {
                    let phonemes = word.split('-').map(ToString::to_string).collect();
                    words.push(WordKind::PhonemeGroup(phonemes));
                }

                _ => (), // Ignore other tokens
            }
        }

        Self(words)
    }

    /// Get the words in the sentence
    #[must_use]
    pub fn words(&self) -> &[WordKind] {
        &self.0
    }
}
impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for word in &self.0 {
            if is_first {
                is_first = false;
            } else {
                write!(f, " ")?;
            }

            match word {
                WordKind::PhonemeGroup(phonemes) => write!(f, "{}", phonemes.join("-"))?,
                WordKind::Number(n) => write!(f, "{n}")?,
            }
        }

        Ok(())
    }
}

/// Represents a set of sentences to be printed on a single line
#[derive(Debug, PartialEq)]
pub struct Line(Vec<Sentence>);
impl Line {
    /// Build a line from a slice of tokens (presumably bounded by a line boundary)
    #[must_use]
    pub fn from_tokens(tokens: &[Token]) -> Self {
        let mut sentences = vec![];
        for tokens in tokens.split(|t| matches!(t, Token::SentenceBoundary)) {
            let sentence = Sentence::from_tokens(tokens);
            sentences.push(sentence);
        }

        Self(sentences)
    }

    /// Get the sentences in the line
    #[must_use]
    pub fn sentences(&self) -> &[Sentence] {
        &self.0
    }
}
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        for sentence in &self.0 {
            if is_first {
                is_first = false;
            } else {
                write!(f, ". ")?;
            }

            write!(f, "{sentence}",)?;
        }

        Ok(())
    }
}

/// Represents a set of lines to be printed in a single block of text
#[derive(Debug, PartialEq)]
pub struct Text(Vec<Line>);
impl Text {
    /// Build a text block from a slice of tokens
    #[must_use]
    pub fn from_tokens(tokens: &[Token]) -> Self {
        let mut lines = vec![];
        for part in tokens.split(|t| matches!(t, Token::LineBoundary)) {
            let line = Line::from_tokens(part);
            lines.push(line);
        }

        Self(lines)
    }

    /// Build a text block from a slice of tokens
    #[must_use]
    pub fn lines(&self) -> &[Line] {
        &self.0
    }

    /// Get a mutable reference to all the words in the text
    pub fn words_mut(&mut self) -> Vec<&mut WordKind> {
        self.0
            .iter_mut()
            .flat_map(|line| line.0.iter_mut())
            .flat_map(|sentence| sentence.0.iter_mut())
            .collect()
    }
}
impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{line}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::tokens_ext::TokensExt;

    #[test]
    fn test_text() {
        let text = "sword of the gods. word of god. son of mark. 2nd potato.";
        let mut tokens = crate::lexer::lex(text);
        tokens.reduce();
        let text = Text::from_tokens(&tokens);

        assert_eq!(
            text,
            Text(vec![Line(vec![
                Sentence(vec![WordKind::PhonemeGroup(vec!["E'sword".to_string()])]),
                Sentence(vec![WordKind::PhonemeGroup(vec!["E'word".to_string()])]),
                Sentence(vec![WordKind::PhonemeGroup(vec!["O'son'mark".to_string()])]),
                Sentence(vec![
                    WordKind::Number(2),
                    WordKind::PhonemeGroup(vec!["potato".to_string()])
                ]),
            ]),])
        );
    }
}
