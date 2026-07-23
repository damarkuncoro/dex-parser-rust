use crate::dex::error::DexError;
use crate::dex::models::raw::RawMethodId;
use crate::dex::readers::DexReader;
use scroll::Pread;

pub struct MethodIdParser;

impl MethodIdParser {
    pub fn parse(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<RawMethodId>, DexError> {
        reader.seek(offset as usize)?;
        let mut methods = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let bytes = reader.read_bytes(8)?;
            let method: RawMethodId = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            methods.push(method);
        }
        Ok(methods)
    }
}
