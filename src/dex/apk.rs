use std::io::{Read, Cursor};
use zip::ZipArchive;
use crate::dex::error::DexError;
use crate::dex::parsers::DexParser;
use crate::dex::models::Dex;

pub struct ApkParser;

impl ApkParser {
    /// Parses all DEX files found inside an APK.
    pub fn parse_apk(buffer: &[u8]) -> Result<Vec<Dex>, DexError> {
        let reader = Cursor::new(buffer);
        let mut archive = ZipArchive::new(reader).map_err(|e| DexError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let mut dex_files = Vec::new();

        // Collect all .dex file names first to avoid borrow checker issues with ZipArchive
        let dex_entry_names: Vec<String> = archive.file_names()
            .filter(|name| name.ends_with(".dex"))
            .map(|s| s.to_string())
            .collect();

        for name in dex_entry_names {
            let mut file = archive.by_name(&name).map_err(|e| DexError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            let mut dex_buffer = Vec::new();
            file.read_to_end(&mut dex_buffer)?;

            // Note: Since each Dex object will have its own internal references to its buffer,
            // we need to leak or manage the life of these buffers.
            // For a simpler CLI tool, we'll keep them owned.
            let parser = DexParser::new(Box::leak(dex_buffer.into_boxed_slice()));
            let dex = parser.parse()?;
            dex_files.push(dex);
        }

        if dex_files.is_empty() {
            return Err(DexError::InvalidMagic); // Or a more specific error like NoDexFilesFound
        }

        Ok(dex_files)
    }
}
