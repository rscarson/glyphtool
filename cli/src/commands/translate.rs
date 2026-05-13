use super::StdinSource;
use libglyphtool::{
    error::EtroisResult,
    lexer::{
        self,
        collections::WordKind,
        phonambulator::{glyphs_to_ipa, PhonemeExt},
    },
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

    /// If provided, the translation will be skipped
    #[arg(short, long)]
    skip_translation: bool,
}
impl Translate {
    pub fn exec(&self) -> EtroisResult<()> {
        let input = match self.path {
            true => Cow::Owned(std::fs::read_to_string(&self.source)?),
            false => Cow::Borrowed(&self.source),
        };

        //
        // Skip translation modes
        if self.skip_translation {
            if self.to_ipa {
                // skip lines starting with #
                let input = input
                    .lines()
                    .filter(|l| !l.starts_with('#') && *l != ".")
                    .collect::<Vec<_>>()
                    .join("\n");

                let ipa = glyphs_to_ipa(&input);
                println!("{ipa}");
                return Ok(());
            } else {
                println!("{input}");
                return Ok(());
            }
        }

        let mut block = lexer::parse(
            &input,
            self.db_path.as_deref(),
            StdinSource::new(self.auto),
            false,
        )?;

        if self.to_ipa {
            for word in block.words_mut() {
                if let Some(ipa) = word.to_ipa() {
                    *word = WordKind::PhonemeGroup(vec![ipa]);
                }
            }
        }

        println!("{block}");
        Ok(())
    }
}
