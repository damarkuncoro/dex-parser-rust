use crate::dex::models::{Field, Proto, DexMetadata};
use crate::dex::models::raw::{RawFieldId, RawProtoId};

pub struct DexLinker;

impl DexLinker {
    /// Links raw field IDs to high-level Field objects.
    pub fn link_fields<'a>(raw_fields: &[RawFieldId], strings: &[&'a str], types: &[&'a str]) -> Vec<Field<'a>> {
        raw_fields.iter().map(|raw| {
            Field {
                class: types.get(raw.class_idx as usize).copied().unwrap_or("<invalid>"),
                type_name: types.get(raw.type_idx as usize).copied().unwrap_or("<invalid>"),
                name: strings.get(raw.name_idx as usize).copied().unwrap_or("<invalid>"),
            }
        }).collect()
    }

    /// Links raw proto IDs to high-level Proto objects.
    pub fn link_protos<'a>(raw_protos: &[RawProtoId], strings: &[&'a str], types: &[&'a str]) -> Vec<Proto<'a>> {
        raw_protos.iter().map(|raw| {
            Proto {
                shorty: strings.get(raw.shorty_idx as usize).copied().unwrap_or("<invalid>"),
                return_type: types.get(raw.return_type_idx as usize).copied().unwrap_or("<invalid>"),
                parameters: Vec::new(), // Parameter linking handled separately due to offset
            }
        }).collect()
    }
}
