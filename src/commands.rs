use crate::{error::Result, lexer, renderer::utilities::glyphs_for};
use clap::{Parser, Subcommand};

mod database;
mod post;
mod render;
mod translate;

/// Run the command line interface
pub fn run() -> Result<()> {
    let args = Args::parse();
    args.commands.exec()
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manage the phoneme database, which maps words to their translations
    Db(database::Db),

    /// Commands to translate text to E'trois phonemes, or IPA
    Translate(translate::Translate),

    /// Commands to render text to images
    Render(render::Render),

    /// Commands to post-process images
    Image(post::Image),

    /// Debug a phoneme by rendering it as ascii
    DebugGlyph {
        /// The phoneme to render
        phoneme: String,

        /// The width of the rendered glyph
        #[arg(short, long, default_value = "0")]
        width: u16,

        /// The height of the rendered glyph
        #[arg(short, long, default_value = "0")]
        height: u16,
    },
}
impl Commands {
    fn exec(&self) -> Result<()> {
        match self {
            Self::Db(command) => command.exec()?,
            Self::Translate(command) => command.exec()?,
            Self::Render(command) => command.exec()?,
            Self::Image(command) => command.exec()?,

            Self::DebugGlyph {
                phoneme,
                width,
                height,
            } => {
                let mut block = lexer::parse(phoneme, None, false)?;
                let words = block.words_mut();
                let Some(glyph) = words.first() else {
                    eprintln!("No glyph found in block");
                    return Ok(());
                };

                let glyphs = glyphs_for(glyph);
                let Some(glyph) = glyphs.iter().flatten().next() else {
                    eprintln!("No glyph found in block");
                    return Ok(());
                };

                let rendered = glyph.as_ref().render_ascii(*width, *height);
                println!("{rendered}");
            }
        }

        Ok(())
    }
}
