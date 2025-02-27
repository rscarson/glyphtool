use clap::{Parser, Subcommand};
use libglyphtool::{
    error::EtroisResult,
    glyphs::ENCODING_TABLE,
    lexer::{
        self,
        phonambulator::{AlwaysAutoSource, PhonambulationSource},
    },
    renderer::{
        bitmap::{Bitmap, ToBitmap},
        GlyphBlockRenderer,
    },
};

mod database;
mod post;
mod render;
mod translate;

/// Run the command line interface
pub fn run() -> EtroisResult<()> {
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

    /// Print all the glyphs in the alphabet  
    /// This is useful for debugging the renderer
    DebugGlyphs {
        /// Only render the glyphs that are not square
        #[arg(long)]
        only_invalid: bool,
    },

    /// Debug the renderer by rendering as ascii
    DebugRenderer {
        /// The phoneme to render
        phoneme: String,
    },
}
impl Commands {
    fn exec(&self) -> EtroisResult<()> {
        match self {
            Self::Db(command) => command.exec()?,
            Self::Translate(command) => command.exec()?,
            Self::Render(command) => command.exec()?,
            Self::Image(command) => command.exec()?,

            Self::DebugGlyphs { only_invalid } => {
                for glyph in ENCODING_TABLE {
                    let bitmap = glyph.to_shrtstop(0, 0).to_bitmap();
                    if *only_invalid && bitmap.is_valid() {
                        continue;
                    }

                    let (width, height) = bitmap.size();
                    let mut canvas = Bitmap::new(width + 2, height + 2);
                    canvas.paste(&bitmap, 1, 1);
                    canvas.invert();

                    let pronounciation = glyph.pronounciation();
                    println!("=====\n{pronounciation}\n{canvas}");
                }
            }

            Self::DebugRenderer { phoneme } => {
                let text = lexer::parse(phoneme, None, AlwaysAutoSource)?;
                println!("{text}");
                let block = GlyphBlockRenderer::new(&text, 0);
                let rendered = block.to_bitmap();
                println!("{rendered}");
            }
        }

        Ok(())
    }
}

pub struct StdinSource(bool);
impl StdinSource {
    pub fn new(auto: bool) -> Self {
        Self(auto)
    }
}
impl PhonambulationSource for StdinSource {
    fn get_next(
        &mut self,
        target: &str,
        auto_suggested: &str,
        err: Option<&str>,
    ) -> EtroisResult<String> {
        if self.0 {
            return Ok("".to_string());
        }

        println!("No mapping found for `{target}` - please enter a set of phoneme groups seperated by '-'");
        println!("For example, 'mad-shik' for magic");
        println!("Vowel Sounds:       [ah/a   ]  [  e/i  ] [   u/uh   ] [ o ]");
        println!("                    [on/apple] [egg/ice] [oops/dunce] [oat]");
        println!("Closed Consonants:  [m] [p] [b] [f]");
        println!(
            "Open Consonants:    [t] [r] [rr | rolled r] [l] [s] [sh] [th] [n] [ng] [k] [d] [z]"
        );
        println!("Special characters: [E' | Deific Mark   ] [O' | Posessive Mark] [A' | Honourific Mark]");
        println!("                    [y- | yellow, yonder] [h- | heather, hoot ] [w- | water, weather ]");
        println!();
        println!("[ j = d-sh ] [ ch = t-sh ] [ v = f ] [ g = k ]");

        println!("Suggested: {auto_suggested}");
        if let Some(err) = err {
            println!("ERROR: Unrecognized phoneme: {err}");
        }
        print!("{target} > ");
        std::io::Write::flush(&mut std::io::stdout()).ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}
