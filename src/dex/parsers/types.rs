use crate::dex::error::DexError;
use crate::dex::readers::DexReader;

pub struct TypeIdParser;

impl TypeIdParser {
    pub fn parse(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<u32>, DexError> {
        reader.seek(offset as usize)?;
        let mut types = Vec::with_capacity(size as usize);
        for _ in 0..size {
            types.push(reader.read_u32()?);
        }
        Ok(types)
    }
}
