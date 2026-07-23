use crate::dex::constants::sizes::CLASS_DEF_ITEM;
use crate::dex::error::DexError;
use crate::dex::models::raw::RawClassDef;
use crate::dex::readers::DexReader;
use scroll::Pread;

pub struct ClassDefParser;

impl ClassDefParser {
    pub fn parse(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<RawClassDef>, DexError> {
        reader.seek(offset as usize)?;
        let mut classes = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let bytes = reader.read_bytes(CLASS_DEF_ITEM)?;
            let class: RawClassDef = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            classes.push(class);
        }
        Ok(classes)
    }
}
