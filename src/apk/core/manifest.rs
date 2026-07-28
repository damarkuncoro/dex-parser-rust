use crate::dex::core::models::Manifest;
use crate::dex::error::DexError;
use super::axml::AxmlParser;

pub struct ManifestParser;

impl ManifestParser {
    /// Parses a binary AndroidManifest.xml buffer.
    pub fn parse(buffer: &[u8]) -> Result<Manifest, DexError> {
        let parser = AxmlParser::new(buffer);
        parser.parse_manifest()
    }
}
