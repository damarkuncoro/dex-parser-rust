use crate::dex::error::DexError;
use crate::dex::parsers::DexParser;
use crate::dex::core::models::Apk;
use crate::analysis::core::config::CompiledConfig;
use super::extractor::ApkExtractor;
use std::sync::Arc;

/// Modular handler for Android APK (ZIP) containers and single DEX files.
pub struct ApkHandler;

impl ApkHandler {
    /// Detects if the buffer starts with the ZIP magic signature.
    pub fn is_apk(buffer: &[u8]) -> bool {
        buffer.starts_with(b"PK\x03\x04")
    }

    /// Primary entry point: Extracts and parses all DEX files from an APK or a single DEX file.
    pub fn process_input(buffer: &[u8]) -> Result<Apk<'_>, DexError> {
        Self::process_with_callback_and_config(buffer, |_| {}, CompiledConfig::compile(Default::default()).unwrap())
    }

    pub fn process_with_config(buffer: &[u8], config: Arc<CompiledConfig>) -> Result<Apk<'_>, DexError> {
        Self::process_with_callback_and_config(buffer, |_| {}, config)
    }

    /// Process input with a progress callback.
    pub fn process_with_callback<F>(buffer: &[u8], callback: F) -> Result<Apk<'_>, DexError>
    where
        F: FnMut(&str)
    {
        Self::process_with_callback_and_config(buffer, callback, CompiledConfig::compile(Default::default()).unwrap())
    }

    pub fn process_with_callback_and_config<F>(buffer: &[u8], mut callback: F, config: Arc<CompiledConfig>) -> Result<Apk<'_>, DexError>
    where
        F: FnMut(&str)
    {
        if Self::is_apk(buffer) {
            ApkExtractor::extract_dex_files_with_config(buffer, callback, config)
        } else {
            callback("Single DEX detected. Parsing...");
            let dex = DexParser::new(buffer).with_config(config).parse_internal()?;
            Ok(Apk::new(vec![dex], vec!["classes.dex".to_string()]))
        }
    }
}
