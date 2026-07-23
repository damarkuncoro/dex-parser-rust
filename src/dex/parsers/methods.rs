use crate::dex::constants::sizes::METHOD_ID_ITEM;
use crate::dex::error::DexError;
use crate::dex::models::header::RawHeader;
use crate::dex::models::raw::RawMethodId;
use scroll::{Endian, Pread};

pub fn parse(
    buffer: &[u8],
    header: &RawHeader,
    strings: &[String],
    types: &[String],
    endian: Endian,
) -> Result<Vec<String>, DexError> {
    let mut methods = Vec::with_capacity(header.method_ids_size as usize);
    for i in 0..header.method_ids_size {
        let off = (header.method_ids_off as usize) + (i as usize * METHOD_ID_ITEM);
        let method_id: RawMethodId = buffer.pread_with(off, endian)?;

        let class_name = types.get(method_id.class_idx as usize).ok_or_else(|| {
            DexError::InvalidIndex(format!("Method class_idx {}", method_id.class_idx))
        })?;
        let method_name = strings.get(method_id.name_idx as usize).ok_or_else(|| {
            DexError::InvalidIndex(format!("Method name_idx {}", method_id.name_idx))
        })?;

        methods.push(format!("{}->{}", class_name, method_name));
    }
    Ok(methods)
}
