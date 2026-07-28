//! Core DEX parsing and data models.

pub mod core;
pub mod error;
pub mod parsers;
pub mod readers;
pub mod validator;
pub use crate::analysis;

pub use error::DexError;
pub use core::models::Dex;
pub use core::linker::DexLinker;
pub use parsers::DexParser;
