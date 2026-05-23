use std::str::FromStr;

use clap::{Parser, Subcommand};
use manager::{
    core::{error::EtroisResult, glyphs::ENCODING_TABLE, renderer::bitmap::Bitmap},
    Filter, Job, PostProcessStep,
};

mod database;
mod post;

/// Run the command line interface
pub fn run() -> EtroisResult<()> {
    let args = Args::parse();
    args.commands.exec()
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    commands: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Phonetic {
        /// The text to translate. If `--file` is provided, this will be treated as a file path
        source: String,

        /// If true, the input is treated as a file path and read from disk
        #[arg(long, short, default_value_t = false)]
        file: bool,

        /// If false, the user will be prompted for each word that doesn't have a mapping in the database.
        ///
        /// If true, the espeak-ng recommended phonetic translation will be used instead.
        #[arg(long)]
        no_stdin: bool,

        /// Path to a translation database. If not provided, `phonemes.db` in the current directory will be used
        #[arg(long)]
        db: Option<String>,
    },

    Ipa {
        /// The text to translate. If `--file` is provided, this will be treated as a file path
        source: String,

        /// The output destination for the IPA output.
        ///
        /// If None, the output will be printed to stdout.
        destination: Option<String>,

        /// If true, the input is treated as a file path and read from disk
        #[arg(long, short, default_value_t = false)]
        file: bool,

        /// Path to a translation database. If not provided, `phonemes.db` in the current directory will be used
        #[arg(long)]
        db: String,
    },

    Render {
        /// The text to render. If `--file` is provided, this will be treated as a file path
        source: String,

        /// The output path for the rendered image
        destination: String,

        /// If true, input is treated as a file path instead of raw text
        #[arg(long, short, default_value_t = false)]
        file: bool,

        /// Optional path to a database file. If not provided `phonemes.db` in the current directory will be used
        #[arg(long)]
        db: Option<String>,

        /// Scale up the image by this factor. Default is 3
        #[arg(short, long, default_value = "3")]
        scale: usize,

        /// Margin around the text in pixels. Default is 5
        #[arg(short, long, default_value = "5")]
        margin: usize,

        /// If false, all glyphs will be rendered with the same height, which is the height of the tallest glyph.
        #[arg(long, default_value_t = false)]
        no_equalize: bool,

        /// If true, line stops will not be included at the end of each line
        #[arg(long, default_value_t = false)]
        no_stops: bool,

        /// Optionally add a visual effect to the text. [sketch, space, granite]
        #[arg(short, long)]
        filter: Option<String>,

        /// Strength of the filter. Default is 1.0
        #[arg(long)]
        filter_strength: Option<f32>,

        /// If true, the source text will be rendered in ascii next to each line
        #[arg(short, long, default_value_t = false)]
        include_translation: bool,

        /// If true, the job will be run in verbose mode, which prints additional information about each step to the console.
        #[arg(long, short, default_value_t = false)]
        verbose: bool,
    },

    RenderAscii {
        /// The text to render. If `--file` is provided, this will be treated as a file path
        source: String,

        /// If true, input is treated as a file path instead of raw text
        #[arg(long, default_value_t = false)]
        file: bool,

        /// Optional path to a database file. If not provided `phonemes.db` in the current directory will be used
        #[arg(long)]
        db: Option<String>,

        /// If false, all glyphs will be rendered with the same height, which is the height of the tallest glyph.
        #[arg(long, default_value_t = false)]
        no_equalize: bool,

        /// If true, line stops will not be included at the end of each line
        #[arg(long, default_value_t = false)]
        no_stops: bool,

        /// If true, the job will be run in verbose mode, which prints additional information about each step to the console.
        #[arg(long, short, default_value_t = false)]
        verbose: bool,
    },

    /// Manage the phoneme database, which maps words to their translations
    Db(database::Db),

    /// Commands to post-process images
    Img(post::Image),

    DebugGlyphs {
        /// If true, only glyphs with invalid bitmaps will be printed
        #[arg(long, default_value_t = false)]
        only_invalid: bool,

        /// The width to render glyphs at for debugging. Default is 20
        #[arg(long, default_value_t = 20)]
        width: usize,

        /// The height to render glyphs at for debugging. Default is 20
        #[arg(long, default_value_t = 20)]
        height: usize,
    },

    Job {
        /// The path to a job file. Jobs are defined in JSON and specify a series of processing steps to apply to an input file, such as translation and rendering.
        job_path: String,

        /// Optional path to a database file. If not provided, `phonemes.db` in the current directory will be used
        #[arg(long)]
        db: Option<String>,

        /// If true, the job will be run in verbose mode, which prints additional information about each step to the console.
        #[arg(long, short, default_value_t = false)]
        verbose: bool,
    },
}
impl Command {
    pub fn exec(&self) -> EtroisResult<()> {
        match self {
            Self::Db(command) => command.exec()?,
            Self::Img(command) => command.exec()?,

            Self::Phonetic {
                source,
                file,
                no_stdin,
                db,
            } => {
                let job = Job::phonetic_translation(source, *file, !*no_stdin);
                if let Err(e) = job.execute(None, None, db.as_deref(), false) {
                    eprintln!("Job failed: {e}");
                }
            }

            Self::Ipa {
                source,
                destination,
                file,
                db,
            } => {
                let job = Job::convert_to_ipa(source, *file, destination.clone());
                if let Err(e) = job.execute(None, None, Some(db), false) {
                    eprintln!("Job failed: {e}");
                }
            }

            Self::Render {
                source,
                destination,
                file,
                db,
                scale,
                margin,
                no_equalize,
                no_stops,
                filter,
                filter_strength,
                include_translation,
                verbose,
            } => {
                let mut post = vec![];
                if let Some(filter) = filter {
                    let Ok(filter) = Filter::from_str(filter) else {
                        eprintln!("Unknown filter: {filter}");
                        return Ok(());
                    };
                    post.push(PostProcessStep::Filter {
                        name: filter,
                        strength: *filter_strength,
                    });
                }
                if scale != &1 {
                    post.push(PostProcessStep::Scale { factor: *scale });
                }

                let job = Job::render(
                    source,
                    *file,
                    destination,
                    *margin,
                    !*no_equalize,
                    !*no_stops,
                    post,
                    *include_translation,
                );
                if let Err(e) = job.execute(None, None, db.as_deref(), *verbose) {
                    eprintln!("Job failed: {e}");
                }
            }

            Self::RenderAscii {
                source,
                file,
                no_equalize,
                no_stops,
                db,
                verbose,
            } => {
                let job = Job::render_ascii(source, *file, !*no_equalize, !*no_stops);
                if let Err(e) = job.execute(None, None, db.as_deref(), *verbose) {
                    eprintln!("Job failed: {e}");
                }
            }

            Self::Job {
                job_path,
                verbose,
                db,
            } => {
                let Ok(mut job) = manager::jobs_from_file(job_path) else {
                    eprintln!("Failed to load job from {job_path}");
                    return Ok(());
                };

                if let Some(db) = db {
                    job.db_path = Some(db.clone());
                }

                if *verbose {
                    println!("Loaded job: {} steps", job.jobs.len());
                }

                if let Err(e) = job.execute_all(*verbose) {
                    eprintln!("Job failed: {e}");
                } else if *verbose {
                    println!("Job completed successfully");
                }
            }

            Self::DebugGlyphs {
                only_invalid,
                width,
                height,
            } => {
                for glyph in ENCODING_TABLE {
                    let bitmap = glyph.render_glyph(*width, *height);
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
        }

        Ok(())
    }
}
