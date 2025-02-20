use crate::{error::Result, lexer, postprocessor::ImageExt, renderer::GlyphBlockRenderer};
use std::borrow::Cow;

#[derive(Debug, clap::Parser)]
pub struct Render {
    /// The text to render. If `--path` is provided, this will be treated as a file path
    source: String,

    /// The output path for the rendered image
    destination: String,

    /// If provided, input is treated as a file path instead of raw text
    #[arg(short, long)]
    path: bool,

    /// Optional path to a database file. If not provided `phonemes.db` is used
    #[arg(short, long)]
    db_path: Option<String>,

    /// Automatically translate words using the autophononimbus. If not provided, the user will be prompted for each new word
    #[arg(short, long)]
    auto: bool,

    /// Scale up the image by this factor. Default is 3
    #[arg(short, long, default_value = "3")]
    scale: usize,

    /// Margin around the text in pixels. Default is 1
    #[arg(short, long, default_value = "1")]
    margin: u16,

    /// Optionally add a visual effect to the text. [sketch, space]
    #[arg(short, long)]
    filter: Option<String>,

    /// Strength of the filter. Default is 1.0
    #[arg(long, default_value = "1.0")]
    filter_stength: f32,

    /// If provided, the output will be opened in the default image viewer
    #[arg(short, long)]
    open: bool,
}
impl Render {
    pub fn exec(&self) -> Result<()> {
        let input = match self.path {
            true => Cow::Owned(std::fs::read_to_string(&self.source)?),
            false => Cow::Borrowed(&self.source),
        };

        println!("Translating {} bytes...", input.len());
        let block = lexer::parse(&input, self.db_path.as_deref(), self.auto)?;
        let renderer = GlyphBlockRenderer::new(&block, self.margin);

        println!("Rendering image...");
        let mut bitmap = renderer.render();

        println!("Scaling image by {}...", self.scale);
        bitmap.scale(self.scale);

        println!("Postprocessing image...");
        let mut image = bitmap.to_grayscale();

        if let Some(filter) = &self.filter {
            println!("Applying {} filter...", filter);

            match filter.as_str() {
                "space" => image.filter_space(self.filter_stength),
                "sketch" => image.filter_sketch(self.filter_stength),
                "granite" => image.filter_granite(self.filter_stength),
                _ => {
                    eprintln!("Unknown filter {filter}");
                }
            }
        }

        println!("Saving image...");
        image.export(&self.destination)?;

        if self.open {
            open::that(&self.destination)?;
        }
        Ok(())
    }
}
