use thiserror::Error;

#[derive(Error, Debug)]
pub enum DexError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid DEX magic number")]
    InvalidMagic,

    #[error("Unsupported DEX version: {0}")]
    InvalidVersion(String),

    #[error("Checksum mismatch: expected {expected:08x}, calculated {calculated:08x}")]
    InvalidChecksum { expected: u32, calculated: u32 },

    #[error("Signature mismatch")]
    InvalidSignature,

    #[error("Invalid offset: {0}")]
    InvalidOffset(String),

    #[error("Index out of bounds: {0}")]
    InvalidIndex(String),

    #[error("Unexpected end of file")]
    UnexpectedEOF,

    #[error("Malformed ULEB128 encoding")]
    MalformedULEB128,

    #[error("Parsing error: {0}")]
    ScrollError(#[from] scroll::Error),

    #[error("Invalid UTF-8 encoding: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Invalid String UTF-8 encoding: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
