use crate::dex::constants::sizes::TYPE_ID_ITEM;
use crate::dex::error::DexError;
use crate::dex::models::header::RawHeader;
use crate::dex::models::type_id::RawTypeId;
use scroll::{Endian, Pread};

pub fn parse(
    buffer: &[u8],
    header: &RawHeader,
    strings: &[String],
    endian: Endian,
) -> Result<Vec<String>, DexError> {
    let mut types = Vec::with_capacity(header.type_ids_size as usize);
    for i in 0..header.type_ids_size {
        let off = (header.type_ids_off as usize) + (i as usize * TYPE_ID_ITEM);
        let type_id: RawTypeId = buffer.pread_with(off, endian)?;

        let descriptor = strings
            .get(type_id.descriptor_idx as usize)
            .ok_or_else(|| {
                DexError::InvalidIndex(format!("Type descriptor_idx {}", type_id.descriptor_idx))
            })?;

        types.push(descriptor.clone());
    }
    Ok(types)
}
