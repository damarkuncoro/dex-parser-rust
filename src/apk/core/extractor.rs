use crate::dex::error::DexError;
use crate::dex::parsers::DexParser;
use crate::dex::core::models::Apk;
use crate::analysis::core::config::CompiledConfig;
use super::archive::ZipManager;
use std::sync::Arc;

pub struct ApkExtractor;

impl ApkExtractor {
    /// Internal: Logic specific to APK (ZIP) extraction.
    pub fn extract_dex_files<'a, F>(buffer: &'a [u8], callback: F) -> Result<Apk<'a>, DexError>
    where
        F: FnMut(&str)
    {
        Self::extract_dex_files_with_config(buffer, callback, CompiledConfig::compile(Default::default()).unwrap())
    }

    pub fn extract_dex_files_with_config<'a, F>(buffer: &'a [u8], mut callback: F, config: Arc<CompiledConfig>) -> Result<Apk<'a>, DexError>
    where
        F: FnMut(&str)
    {
        callback("APK detected. Opening archive...");
        let mut zip = ZipManager::new(buffer)?;

        let dex_entry_names: Vec<String> = zip.get_file_names()
            .into_iter()
            .filter(|name| name.ends_with(".dex"))
            .collect();

        let mut dex_files = Vec::new();
        let mut dex_names = Vec::new();
        let total = dex_entry_names.len();

        for (i, name) in dex_entry_names.into_iter().enumerate() {
            callback(&format!("Parsing {} ({} of {})...", name, i + 1, total));

            let dex_buffer = match zip.extract_file(&name) {
                Ok(buf) => buf,
                Err(e) => {
                    callback(&format!("Warning: Failed to extract {}: {}", name, e));
                    continue;
                }
            };

            // Maintain Zero-Copy lifetimes of 'static for the extracted DEX.
            let leaked_buffer: &'static [u8] = Box::leak(dex_buffer.into_boxed_slice());

            match DexParser::new(leaked_buffer).with_config(config.clone()).parse_internal() {
                Ok(dex) => {
                    dex_files.push(dex);
                    dex_names.push(name);
                }
                Err(e) => {
                    callback(&format!("Warning: Failed to parse {}: {}", name, e));
                }
            }
        }

        let manifest = if zip.get_file_names().contains(&"AndroidManifest.xml".to_string()) {
            callback("Extracting AndroidManifest.xml...");
            if let Ok(buf) = zip.extract_file("AndroidManifest.xml") {
                super::manifest::ManifestParser::parse(&buf).ok()
            } else {
                None
            }
        } else {
            None
        };

        if dex_files.is_empty() {
            return Err(DexError::InvalidMagic);
        }

        callback("Finalizing APK context...");
        Ok(Apk::new_with_manifest(dex_files, dex_names, manifest))
    }
}
