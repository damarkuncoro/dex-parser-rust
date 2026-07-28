use std::io::{Read, Cursor};
use zip::ZipArchive;
use crate::dex::error::DexError;

pub struct ZipManager<'a> {
    archive: ZipArchive<Cursor<&'a [u8]>>,
}

impl<'a> ZipManager<'a> {
    pub fn new(buffer: &'a [u8]) -> Result<Self, DexError> {
        let reader = Cursor::new(buffer);
        let archive = ZipArchive::new(reader)
            .map_err(|e| DexError::ZipError(e.to_string()))?;
        Ok(Self { archive })
    }

    pub fn get_file_names(&self) -> Vec<String> {
        self.archive.file_names()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn extract_file(&mut self, name: &str) -> Result<Vec<u8>, DexError> {
        let mut file = self.archive.by_name(name)
            .map_err(|e| DexError::ZipError(e.to_string()))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
