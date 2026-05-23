//! Batch job runner for the high etrois toolchain.
//!
//! Jobs are defined in JSON and executed sequentially.
//!
//! Supports:
//! - phonetic translation
//! - IPA conversion
//! - glyph rendering
//! - image postprocessing
//!
//! # Example
//!
//! ```json
//! {
//!   "jobs": [
//!     {
//!       "source": {
//!         "Text": "ki trois"
//!       },
//!       "steps": [
//!         {
//!           "ConvertToIpa": {
//!             "destination": "Stdout"
//!           }
//!         }
//!       ]
//!     }
//!   ]
//! }
//! ```
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use libglyphtool::{
    error::{Error, EtroisResult},
    lexer::{self, collections::Text, phonambulator::glyphs_to_ipa},
    postprocessor::OutputImage,
    renderer::{GlyphBlockOptions, GlyphBlockRenderer, bitmap::ToBitmap},
};

pub use libglyphtool as core;

use crate::source::StdinSource;

mod source;

fn serde_default_true() -> bool {
    true
}
fn serde_default_false() -> bool {
    false
}
fn serde_default_5() -> usize {
    5
}

/// Input text source.
///
/// `Text` uses inline content.
/// `File` reads from disk.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum TextSource {
    /// Inline source text.
    Text(String),

    /// File path to read source text from.
    File(String),
}
impl TextSource {
    /// Creates a new `TextSource` from a string and a flag indicating whether it's a file path.
    #[must_use]
    pub fn new(source: &str, is_file: bool) -> Self {
        if is_file {
            Self::File(source.to_string())
        } else {
            Self::Text(source.to_string())
        }
    }

    /// Reads the source into memory.
    ///
    /// If `from_dir` is provided, file paths are resolved relative to it.
    pub fn read(&self, from_dir: Option<&str>) -> EtroisResult<String> {
        match self {
            Self::Text(text) => Ok(text.clone()),
            Self::File(path) => {
                if let Some(dir) = from_dir {
                    let full_path = std::path::Path::new(dir).join(path);
                    Ok(std::fs::read_to_string(full_path)?)
                } else {
                    Ok(std::fs::read_to_string(path)?)
                }
            }
        }
    }

    /// Returns true if this source is a file path that ends with `.ruh`
    #[must_use]
    pub fn is_ruh(&self) -> bool {
        matches!(self, Self::File(path) if std::path::Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("ruh")))
    }
}

/// Image postprocessing step.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PostProcessStep {
    /// Applies a filter with the given strength.
    Filter {
        /// The filter to apply.
        name: Filter,

        /// The strength of the filter. If not provided, defaults to 1.0
        strength: Option<f32>,
    },

    /// Scales the rendered image.
    Scale {
        /// The scaling factor. For example, a factor of 2 will double the width and height of the image.
        factor: usize,
    },
}
impl PostProcessStep {
    /// Applies the postprocessing step to an image.
    pub fn apply(&self, bitmap: &mut OutputImage, verbose: bool) {
        match self {
            PostProcessStep::Filter { name, strength } => {
                let strength = strength.unwrap_or(1.0);
                match name {
                    Filter::Sketch => bitmap.filter_sketch(strength, verbose),
                    Filter::Space => bitmap.filter_space(strength, verbose),
                    Filter::Granite => bitmap.filter_granite(strength, verbose),
                }
            }
            PostProcessStep::Scale { factor } => bitmap.scale(*factor),
        }
    }
}

/// Built-in image filters.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Filter {
    /// Sepia with a subtle blur
    Sketch,

    /// Inverted colors with a heavy blur and colour noise
    Space,

    /// Edge detection embossed effect on slate texture
    Granite,
}

impl std::str::FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sketch" => Ok(Self::Sketch),
            "space" => Ok(Self::Space),
            "granite" => Ok(Self::Granite),
            _ => Err(format!("unknown filter: {s}")),
        }
    }
}

/// A single processing step in a job pipeline.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JobStep {
    /// Runs phonetic translation into E'trois glyphs.
    PhoneticTranslation {
        /// If true, the user will be prompted for phonetic translations and any unmapped words use the epspeak recommendation
        #[serde(default = "serde_default_true")]
        allow_stdin_input: bool,
    },

    /// Converts the phonetic glyphs to IPA symbols and writes them to the destination.
    Ipa {
        /// The destination to write the IPA output to.
        /// Stdout if None, otherwise treated as a file path.
        destination: Option<String>,
    },

    /// Renders translated glyphs into an image.
    Render {
        /// Output image path.
        image_out_path: String,

        /// Margin size in pixels.
        #[serde(default = "serde_default_5")]
        margins: usize,

        /// Equalizes vertical stack heights.
        #[serde(default = "serde_default_true")]
        equalize_stack_heights: bool,

        /// Includes line stop markers.
        #[serde(default = "serde_default_true")]
        add_line_stops: bool,

        /// Postprocessing operations.
        post_process_steps: Vec<PostProcessStep>,

        /// Renders source translation beneath glyphs.
        #[serde(default = "serde_default_false")]
        render_translation: bool,
    },

    /// Renders translated glyphs into ascii art for debugging.
    ///
    /// Prints the ascii art to stdout.
    RenderAscii {
        /// Equalizes vertical stack heights.
        #[serde(default = "serde_default_true")]
        equalize_stack_heights: bool,

        /// Includes line stop markers.
        #[serde(default = "serde_default_true")]
        add_line_stops: bool,
    },
}
impl JobStep {
    fn load_block_if_needed(&self, state: &mut JobState, verbose: bool) -> EtroisResult<()> {
        if state.block.is_none() {
            if verbose {
                println!("Loading block from input for step {self:?}...");
            }
            state.block = Some(lexer::parse(
                &state.input,
                state.db_path,
                StdinSource::new(false),
                true,
            )?);
        }

        Ok(())
    }

    /// Applies the processing step to the job state.
    #[allow(clippy::too_many_lines)]
    fn apply(&self, state: &mut JobState, verbose: bool) -> EtroisResult<()> {
        match self {
            Self::PhoneticTranslation { allow_stdin_input } => {
                state.block = Some(lexer::parse(
                    &state.input,
                    state.db_path,
                    StdinSource::new(!*allow_stdin_input),
                    false,
                )?);
            }

            Self::Ipa { destination } => {
                self.load_block_if_needed(state, verbose)?;
                let block = state
                    .block
                    .as_ref()
                    .expect("Block should be loaded at this point");

                let glyphs = block.to_string();
                let ipa = glyphs_to_ipa(&glyphs);

                match destination {
                    None => println!("{ipa}"),
                    Some(path) => {
                        if let Some(dir) = state.output_dir {
                            let full_path = std::path::Path::new(dir).join(path);
                            std::fs::write(full_path, ipa)?;
                        } else {
                            std::fs::write(path, ipa)?;
                        }
                    }
                }
            }

            Self::Render {
                image_out_path,
                margins,
                equalize_stack_heights,
                add_line_stops,
                post_process_steps,
                render_translation,
            } => {
                self.load_block_if_needed(state, verbose)?;
                let block = state
                    .block
                    .as_ref()
                    .expect("Block should be loaded at this point");

                let dest_kind = match std::path::Path::new(image_out_path)
                    .extension()
                    .and_then(|x| x.to_str())
                {
                    Some("png") => ImageKind::Png,
                    Some("webp") => ImageKind::Webp,
                    _ => {
                        return Err(Error::Other(
                            "Unsupported output format. Supported formats are: png, webp"
                                .to_string(),
                        ));
                    }
                };

                let options = GlyphBlockOptions {
                    margin: *margins,
                    equalize_heights: *equalize_stack_heights,
                    include_stop: *add_line_stops,
                    include_translation: *render_translation,
                };

                if verbose {
                    println!("Rendering with options: {options:?}");
                }
                let renderer = GlyphBlockRenderer::new(block, options);

                if verbose {
                    println!("Rendering image...");
                }
                let bitmap = renderer.to_bitmap();

                if verbose {
                    println!("Postprocessing image...");
                }
                let mut image = OutputImage::new_grayscale(&bitmap);

                for step in post_process_steps {
                    if verbose {
                        println!("Applying postprocess step: {step:?}");
                    }
                    step.apply(&mut image, verbose);
                }

                if verbose {
                    println!("Saving image...");
                }
                let bytes = match dest_kind {
                    ImageKind::Png => image.into_png()?,
                    ImageKind::Webp => image
                        .into_webp(50.0)
                        .ok_or(Error::Other("Failed to encode image".to_string()))?,
                };

                let out_path = if let Some(dir) = state.output_dir {
                    std::path::Path::new(dir).join(image_out_path)
                } else {
                    std::path::PathBuf::from(image_out_path)
                };

                std::fs::write(out_path, bytes)?;
            }

            Self::RenderAscii {
                equalize_stack_heights,
                add_line_stops,
            } => match &state.block {
                None => {
                    Self::PhoneticTranslation {
                        allow_stdin_input: false,
                    }
                    .apply(state, verbose)?;
                    self.apply(state, verbose)?;
                }

                Some(block) => {
                    let options = GlyphBlockOptions {
                        margin: 0,
                        equalize_heights: *equalize_stack_heights,
                        include_stop: *add_line_stops,
                        include_translation: false,
                    };

                    if verbose {
                        println!("Rendering with options: {options:?}");
                    }
                    let renderer = GlyphBlockRenderer::new(block, options);

                    if verbose {
                        println!("Rendering image...");
                    }
                    let bitmap = renderer.to_bitmap();
                    println!("{bitmap}");
                }
            },
        }
        Ok(())
    }
}

/// A single executable job.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Job {
    /// Input source.
    pub source: TextSource,

    /// Ordered processing steps.
    pub steps: Vec<JobStep>,
}
impl Job {
    /// Helper function to create a `PhoneticTranslation` job with common options.
    #[must_use]
    pub fn phonetic_translation(source: &str, is_file: bool, allow_stdin_input: bool) -> Self {
        let source = TextSource::new(source, is_file);
        let step = JobStep::PhoneticTranslation { allow_stdin_input };

        Self {
            source,
            steps: vec![step],
        }
    }

    /// Helper function to create a `ConvertToIpa` job with common options.
    #[must_use]
    pub fn convert_to_ipa(source: &str, is_file: bool, destination: Option<String>) -> Self {
        let source = TextSource::new(source, is_file);
        let step = JobStep::Ipa { destination };

        Self {
            source,
            steps: vec![step],
        }
    }

    /// Helper function to create a `Render` job with common options.
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    #[must_use]
    pub fn render(
        source: &str,
        is_file: bool,
        image_out_path: &str,
        margins: usize,
        equalize_stack_heights: bool,
        add_line_stops: bool,
        post_process_steps: Vec<PostProcessStep>,
        render_translation: bool,
    ) -> Self {
        let source = TextSource::new(source, is_file);
        let step = JobStep::Render {
            image_out_path: image_out_path.to_string(),
            margins,
            equalize_stack_heights,
            add_line_stops,
            post_process_steps,
            render_translation,
        };

        Self {
            source,
            steps: vec![step],
        }
    }

    /// Helper function to create a `RenderAscii` job with common options.
    #[must_use]
    pub fn render_ascii(
        source: &str,
        is_file: bool,
        equalize_stack_heights: bool,
        add_line_stops: bool,
    ) -> Self {
        let source = TextSource::new(source, is_file);
        let step = JobStep::RenderAscii {
            equalize_stack_heights,
            add_line_stops,
        };

        Self {
            source,
            steps: vec![step],
        }
    }

    /// Executes the job.
    pub fn execute(
        &self,
        source_dir: Option<&str>,
        output_dir: Option<&str>,
        db_path: Option<&str>,
        verbose: bool,
    ) -> EtroisResult<()> {
        if verbose {
            println!("Starting job with source: {:?}", self.source);
        }
        let input = self.source.read(source_dir)?;

        let mut state = JobState {
            input,
            block: None,

            output_dir,
            db_path,
        };

        for step in &self.steps {
            step.apply(&mut state, verbose)?;
        }

        Ok(())
    }
}

/// A collection of jobs to execute.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Jobs {
    /// Base directory for source files.
    pub source_dir: Option<String>,

    /// Base directory for output files.
    pub output_dir: Option<String>,

    /// Translation database path.
    pub db_path: Option<String>,

    /// Jobs to execute.
    pub jobs: Vec<Job>,
}
impl Jobs {
    /// Executes all jobs sequentially.
    pub fn execute_all(&self, verbose: bool) -> EtroisResult<()> {
        for job in &self.jobs {
            if verbose {
                println!("Executing job: {job:?}");
            }

            job.execute(
                self.source_dir.as_deref(),
                self.output_dir.as_deref(),
                self.db_path.as_deref(),
                verbose,
            )?;
        }
        Ok(())
    }
}

struct JobState<'src> {
    pub input: String,
    pub block: Option<Text>,

    pub output_dir: Option<&'src str>,
    pub db_path: Option<&'src str>,
}

/// Helper function to read jobs from a JSON string
pub fn jobs_from_json(json: &str) -> Result<Jobs, serde_json::Error> {
    let jobs = serde_json::from_str(json)?;
    Ok(jobs)
}

/// Helper function to read jobs from a YAML string
pub fn jobs_from_yaml(yaml: &str) -> Result<Jobs, serde_yaml::Error> {
    let jobs = serde_yaml::from_str(yaml)?;
    Ok(jobs)
}

/// Helper function to read jobs from a JSON or YAML file
pub fn jobs_from_file(path: &str) -> Result<Jobs, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|x| x.to_str())
        .unwrap_or("")
        .to_lowercase();
    let mut jobs = match ext.as_str() {
        "json" => jobs_from_json(&json)?,
        "yaml" | "yml" => jobs_from_yaml(&json)?,
        _ => Err(std::io::Error::other("Unsupported Filetype"))?,
    };

    // Set the *_dir fields to the parent directory of the job file if they are not already set
    // and make the existing paths relative to the job file directory if set and not absolute
    let job_dir = std::path::Path::new(path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    match &mut jobs.source_dir {
        None => jobs.source_dir = Some(job_dir.to_string_lossy().to_string()),
        Some(dir) => {
            let dir_path = std::path::Path::new(dir);
            if !dir_path.is_absolute() {
                let full_path = job_dir.join(dir_path);
                *dir = full_path.to_string_lossy().to_string();
            }
        }
    }
    match &mut jobs.output_dir {
        None => jobs.output_dir = Some(job_dir.to_string_lossy().to_string()),
        Some(dir) => {
            let dir_path = std::path::Path::new(dir);
            if !dir_path.is_absolute() {
                let full_path = job_dir.join(dir_path);
                *dir = full_path.to_string_lossy().to_string();
            }
        }
    }

    Ok(jobs)
}

#[derive(Debug, Clone)]
enum ImageKind {
    Png,
    Webp,
}
