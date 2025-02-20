use crate::{
    error::Result,
    lexer::{self, collections::WordKind},
    renderer::utilities::ipa_string_for,
};
use std::borrow::Cow;

#[derive(Debug, clap::Parser)]
pub struct Translate {
    /// The text to translate. If `--path` is provided, this will be treated as a file path
    source: String,

    /// Optional path to a database file. If not provided `phonemes.db` is used
    #[arg(short, long)]
    db_path: Option<String>,

    /// Automatically translate words using the autophononimbus. If not provided, the user will be prompted for each new word
    #[arg(short, long)]
    auto: bool,

    /// If provided, input is treated as a file path instead of raw text
    #[arg(short, long)]
    path: bool,

    /// If provided, the output will be IPA symbols instead of phonemes
    #[arg(short, long)]
    to_ipa: bool,
}
impl Translate {
    pub fn exec(&self) -> Result<()> {
        let input = match self.path {
            true => Cow::Owned(std::fs::read_to_string(&self.source)?),
            false => Cow::Borrowed(&self.source),
        };
        let mut block = lexer::parse(&input, self.db_path.as_deref(), self.auto)?;

        if self.to_ipa {
            for word in block.words_mut() {
                let ipa = ipa_string_for(word);
                *word = WordKind::PhonemeGroup(vec![ipa]);
            }
        }

        println!("{block}");
        Ok(())
    }
}
