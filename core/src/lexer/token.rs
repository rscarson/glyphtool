use logos::Logos;

/// Set of tokens representible in old high E'Trois
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    /// The deific mark (E')
    #[regex(" *of god")]
    #[regex(" *of the gods")]
    DeificModifier,

    /// The posessive mark (O')
    #[regex(" *of *")]
    #[regex(" *belonging to *")]
    OwnershipModifier,

    /// A number (with ordinal suffix removed)
    #[regex("([0-9]+)(st|nd|rd|th)?", |lex| {
        let slice = lex.slice().trim_end_matches(['s', 't', 'n', 'd', 'r', 'h']);
        slice.parse().ok()
    })]
    Number(u16),

    /// A word
    #[regex("[a-zA-Z'-]+", |lex| lex.slice().to_string())]
    Word(String),

    /// Whitespace
    #[regex(r" +")]
    WordBoundary,

    /// A line boundary
    #[regex("\n\r|\r\n|[\r\n]")]
    LineBoundary,

    /// A comment
    #[regex(r"(#|//).*(\n\r|\r\n|[\r\n])?")]
    Comment,

    /// A line describing the content of the next non comment/source line, for use in rendering
    #[regex(r"> .*(\n\r|\r\n|[\r\n])?", |lex| {
        lex.slice().trim_start_matches('>').trim().to_string()
    })]
    SourceTextLine(String),

    /// End of sentence punctuation
    #[regex("[.,;:!?] *")]
    SentenceBoundary,

    /// It's just :: again but used for cartouches.
    #[regex(r"\|")]
    Cartouche,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexing() {
        let mut lex = Token::lexer("sword of the gods. word of god. son of mark. 2nd potato.");

        assert_eq!(lex.next(), Some(Ok(Token::Word("sword".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::DeificModifier)));
        assert_eq!(lex.next(), Some(Ok(Token::SentenceBoundary)));

        assert_eq!(lex.next(), Some(Ok(Token::Word("word".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::DeificModifier)));
        assert_eq!(lex.next(), Some(Ok(Token::SentenceBoundary)));

        assert_eq!(lex.next(), Some(Ok(Token::Word("son".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::OwnershipModifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Word("mark".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::SentenceBoundary)));

        assert_eq!(lex.next(), Some(Ok(Token::Number(2))));
        assert_eq!(lex.next(), Some(Ok(Token::WordBoundary)));
        assert_eq!(lex.next(), Some(Ok(Token::Word("potato".to_string()))));
        assert_eq!(lex.next(), Some(Ok(Token::SentenceBoundary)));

        assert_eq!(lex.next(), None);
    }
}
