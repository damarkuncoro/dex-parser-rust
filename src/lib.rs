//! # dex-parser-rust 🚀
//!
//! A high-performance, modular, and parallel Android DEX/APK parsing engine written in Rust.
//! Designed as a modern, safe, and significantly faster alternative to traditional utilities like `dexdump`.

pub mod dex;
pub mod apk;
pub mod analysis;
pub mod exporter;
pub mod ffi;
pub mod wasm;

// Public API Re-exports
pub use crate::dex::parsers::DexParser;
pub use crate::apk::ApkHandler;
pub use crate::dex::core::models::{Dex, Apk};
pub use crate::dex::error::DexError;
pub use crate::analysis::core::config::{AnalysisConfig, CompiledConfig};
