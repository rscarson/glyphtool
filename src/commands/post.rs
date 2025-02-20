use crate::{
    error::Result,
    postprocessor::{ImageExt, OutputImage},
};
use clap::Subcommand;

#[derive(Debug, clap::Parser)]
pub struct Image {
    #[clap(subcommand)]
    command: Command,

    in_path: String,
    out_path: String,
}
impl Image {
    pub fn exec(&self) -> Result<()> {
        self.command.exec(&self.in_path, &self.out_path)
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Scale the image by a factor
    Scale { factor: usize },

    /// Apply a filter to the image
    Filter {
        filter: String,

        #[clap(long, default_value = "1.0")]
        strength: f32,
    },
}
impl Command {
    pub fn exec(&self, in_path: &str, out_path: &str) -> Result<()> {
        let mut image = OutputImage::load(in_path)?;
        match self {
            Self::Scale { factor, .. } => {
                image.scale(*factor);
            }

            Self::Filter {
                filter, strength, ..
            } => match filter.as_str() {
                "sketch" => image.filter_sketch(*strength),
                "space" => image.filter_space(*strength),
                "granite" => image.filter_granite(*strength),

                _ => {
                    eprintln!("Unknown filter: {}", filter);
                    return Ok(());
                }
            },
        }

        image.export(out_path)?;
        Ok(())
    }
}
