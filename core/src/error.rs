//! Defines the error type for the E'trois glyphtool
use thiserror::Error;

/// Error type for the library
#[derive(Error, Debug)]
pub enum Error {
    /// Error type for the database module
    #[error("SQlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Error type for the lexer
    #[error("Could not parse input at: {0}")]
    Lexer(String),

    /// Error type for the renderer
    #[error("Image error: {0}")]
    Renderer(#[from] image::error::ImageError),

    /// IO error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error type for the model (load / save)
    #[error("Burn error: {0}")]
    Recorder(#[from] burn::record::RecorderError),

    /// Unknown error
    #[error("Error: {0}")]
    Other(String),
}

/// Result type for the E'trois glyphtool
pub type EtroisResult<T> = Result<T, Error>;

impl From<burn::tensor::DataError> for Error {
    fn from(value: burn::tensor::DataError) -> Self {
        Error::Other(format!("{value:?}"))
    }
}
