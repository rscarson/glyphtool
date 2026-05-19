use super::StdinSource;
use libglyphtool::{
    error::{Error, EtroisResult},
    lexer,
    postprocessor::OutputImage,
    renderer::{bitmap::ToBitmap, GlyphBlockOptions, GlyphBlockRenderer},
};
use std::borrow::Cow;

enum OutputKind {
    Png,
    Webp,
}

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

    /// Margin around the text in pixels. Default is 5
    #[arg(short, long, default_value = "5")]
    margin: usize,

    /// If false, all glyphs will be rendered with the same height, which is the height of the tallest glyph.
    #[arg(long, default_value = "false")]
    no_equalize_heights: bool,

    /// If true, line stops will not be included at the end of each line
    #[arg(long, default_value = "false")]
    no_line_stops: bool,

    /// Optionally add a visual effect to the text. [sketch, space]
    #[arg(short, long)]
    filter: Option<String>,

    /// Strength of the filter. Default is 1.0
    #[arg(long, default_value = "1.0")]
    filter_stength: f32,

    /// Skip the translation step. For writing in native E'trois
    /// Automatic for files ending in .ruh, which are already in E'trois
    #[arg(long)]
    skip_translation: bool,

    /// If provided, the output will be opened in the default image viewer
    #[arg(short, long)]
    open: bool,
}
impl Render {
    pub fn exec(&self) -> EtroisResult<()> {
        let input = match self.path {
            true => Cow::Owned(std::fs::read_to_string(&self.source)?),
            false => Cow::Borrowed(&self.source),
        };

        let destination = std::path::Path::new(&self.destination);
        let kind = match destination.extension() {
            Some(x) if x == "png" => OutputKind::Png,
            Some(x) if x == "webp" => OutputKind::Webp,
            _ => {
                eprintln!("Unsupported output format. Supported formats are: png, webp");
                return Err(Error::Other("Unsupported output format".to_string()));
            }
        };

        let mut skip_translation = self.skip_translation;
        if self.path && self.source.ends_with(".ruh") {
            skip_translation = true; // .ruh files are already in E'trois, so we can skip translation
        }

        println!("Translating {} bytes...", input.len());
        let block = lexer::parse(
            &input,
            self.db_path.as_deref(),
            StdinSource::new(self.auto),
            skip_translation,
        )?;

        let options = GlyphBlockOptions {
            margin: self.margin,
            equalize_heights: !self.no_equalize_heights,
            include_stop: !self.no_line_stops,
        };
        let renderer = GlyphBlockRenderer::new(&block, options);

        println!("Rendering image...");
        let bitmap = renderer.to_bitmap();

        println!("Postprocessing image...");
        let mut image = OutputImage::new_grayscale(&bitmap);

        println!("Scaling image by {}...", self.scale);
        image.scale(self.scale);

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
        let bytes = match kind {
            OutputKind::Webp => image
                .into_webp(50.0)
                .ok_or(Error::Other("Failed to encode image".to_string()))?,
            OutputKind::Png => image.into_png()?,
        };
        std::fs::write(&self.destination, bytes)?;

        if self.open {
            open::that(&self.destination)?;
        }
        Ok(())
    }
}
