//! Core DEX parsing and data models.

pub mod core;
pub mod display;
pub mod error;
pub mod parsers;
pub mod readers;
pub mod validator;
pub mod linker;
pub mod apk;
pub mod analysis;

pub use error::DexError;
pub use core::models::Dex;
pub use parsers::DexParser;
