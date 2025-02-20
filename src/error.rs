use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("SQlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Could not parse input at: {0}")]
    Lexer(String),

    #[error("Image error: {0}")]
    Renderer(#[from] image::error::ImageError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
