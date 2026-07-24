use std::io::{Read, Cursor};
use zip::ZipArchive;
use crate::dex::error::DexError;
use crate::dex::parsers::DexParser;
use crate::dex::models::Apk;

pub struct ApkParser;

impl ApkParser {
    /// Parses all DEX files found inside an APK and returns a unified Apk model.
    pub fn parse_apk(buffer: &[u8]) -> Result<Apk<'_>, DexError> {
        let reader = Cursor::new(buffer);
        let mut archive = ZipArchive::new(reader).map_err(|e| DexError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let mut dex_files = Vec::new();

        let dex_entry_names: Vec<String> = archive.file_names()
            .filter(|name| name.ends_with(".dex"))
            .map(|s| s.to_string())
            .collect();

        for name in dex_entry_names {
            let mut file = archive.by_name(&name).map_err(|e| DexError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            let mut dex_buffer = Vec::new();
            file.read_to_end(&mut dex_buffer)?;

            // To maintain Zero-copy across the app, we need the buffer to live long enough.
            // For APK extraction, we leak the individual DEX buffers into the heap.
            let leaked_buffer: &'static [u8] = Box::leak(dex_buffer.into_boxed_slice());
            let dex = DexParser::parse(leaked_buffer)?;
            dex_files.push(dex);
        }

        if dex_files.is_empty() {
            return Err(DexError::InvalidMagic);
        }

        Ok(Apk::new(dex_files))
    }
}
