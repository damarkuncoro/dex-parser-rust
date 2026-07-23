use crate::dex::error::DexError;
use crate::dex::models::raw::RawFieldId;
use crate::dex::readers::DexReader;
use scroll::Pread;

pub struct FieldIdParser;

impl FieldIdParser {
    pub fn parse(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<RawFieldId>, DexError> {
        reader.seek(offset as usize)?;
        let mut items = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let bytes = reader.read_bytes(8)?;
            let item: RawFieldId = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            items.push(item);
        }
        Ok(items)
    }
}
