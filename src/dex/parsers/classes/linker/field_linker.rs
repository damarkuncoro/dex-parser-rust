use crate::dex::models::{EncodedField, Field};
use crate::dex::parsers::class_data;
use crate::dex::utils::access_flags::translate_access_flags;

pub fn link_fields<'a>(
    raw_fields: &[class_data::RawEncodedField],
    fields: &[Field<'a>],
) -> Vec<EncodedField<'a>> {
    let mut last_idx = 0u64;
    raw_fields.iter().map(|raw| {
        let field_idx = (last_idx + raw.field_idx_diff) as usize;
        last_idx = field_idx as u64;
        let field_info = fields.get(field_idx).cloned().unwrap_or_else(|| {
            Field {
                class: "<invalid>",
                type_name: "<invalid>",
                name: "<invalid>",
            }
        });
        EncodedField {
            name: field_info.name,
            type_name: field_info.type_name,
            access_flags: raw.access_flags as u32,
            access_flags_text: translate_access_flags(raw.access_flags as u32, false),
        }
    }).collect()
}
