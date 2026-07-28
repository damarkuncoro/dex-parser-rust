use crate::dex::error::DexError;
use crate::dex::core::models::raw::RawProtoId;
use crate::dex::readers::DexReader;
use scroll::Pread;

pub struct ProtoIdParser;

impl ProtoIdParser {
    pub fn parse(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<RawProtoId>, DexError> {
        reader.seek(offset as usize)?;
        let mut items = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let bytes = reader.read_bytes(12)?;
            let item: RawProtoId = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            items.push(item);
        }
        Ok(items)
    }
}
