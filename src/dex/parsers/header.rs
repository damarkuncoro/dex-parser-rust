use crate::dex::error::DexError;
use crate::dex::models::header::RawHeader;
use crate::dex::readers::DexReader;
use scroll::Pread;

pub struct HeaderParser;

impl HeaderParser {
    pub fn parse(reader: &mut DexReader) -> Result<RawHeader, DexError> {
        reader.seek(0)?;
        let bytes = reader.read_bytes(112)?;
        let header: RawHeader = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
        Ok(header)
    }
}
