use crate::dex::constants::sizes::FIELD_ID_ITEM;
use crate::dex::error::DexError;
use crate::dex::models::field::Field;
use crate::dex::models::header::RawHeader;
use crate::dex::models::raw::RawFieldId;
use scroll::{Endian, Pread};

pub fn parse(
    buffer: &[u8],
    header: &RawHeader,
    strings: &[String],
    types: &[String],
    endian: Endian,
) -> Result<Vec<Field>, DexError> {
    let mut fields = Vec::with_capacity(header.field_ids_size as usize);
    for i in 0..header.field_ids_size {
        let off = (header.field_ids_off as usize) + (i as usize * FIELD_ID_ITEM);
        let raw: RawFieldId = buffer.pread_with(off, endian)?;

        fields.push(Field {
            class: types
                .get(raw.class_idx as usize)
                .cloned()
                .unwrap_or_default(),
            type_name: types
                .get(raw.type_idx as usize)
                .cloned()
                .unwrap_or_default(),
            name: strings
                .get(raw.name_idx as usize)
                .cloned()
                .unwrap_or_default(),
        });
    }
    Ok(fields)
}
