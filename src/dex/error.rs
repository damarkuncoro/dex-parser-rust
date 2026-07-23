use thiserror::Error;

#[derive(Error, Debug)]
pub enum DexError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    Scroll(#[from] scroll::Error),

    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Index out of bounds: {0}")]
    InvalidIndex(String),
}
