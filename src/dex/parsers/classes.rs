use crate::dex::constants::{sizes::CLASS_DEF_ITEM, NO_INDEX};
use crate::dex::error::DexError;
use crate::dex::models::{header::RawHeader, raw::RawClassDef, Class, Proto};
use crate::dex::parsers::class_data;
use crate::dex::parsers::traits::DexResolver;
use crate::dex::utils::access_flags::translate_access_flags;
use rayon::prelude::*;
use scroll::{Endian, Pread};

pub fn parse<R: DexResolver + Sync>(
    buffer: &[u8],
    header: &RawHeader,
    protos: &[Proto],
    resolver: &R,
    endian: Endian,
) -> Result<Vec<Class>, DexError> {
    (0..header.class_defs_size)
        .into_par_iter()
        .map(|i| {
            let off = (header.class_defs_off as usize) + (i as usize * CLASS_DEF_ITEM);
            let class_def: RawClassDef = buffer
                .pread_with(off, endian)
                .map_err(DexError::Scroll)?;

            let name = resolver.resolve_type(class_def.class_idx).ok_or_else(|| {
                DexError::InvalidIndex(format!("Class class_idx {}", class_def.class_idx))
            })?;

            let superclass = if class_def.superclass_idx == NO_INDEX {
                "None".to_string()
            } else {
                resolver
                    .resolve_type(class_def.superclass_idx)
                    .ok_or_else(|| {
                        DexError::InvalidIndex(format!(
                            "Class superclass_idx {}",
                            class_def.superclass_idx
                        ))
                    })?
            };

            let mut interfaces = Vec::new();
            if class_def.interfaces_off != 0 {
                let mut curr = class_def.interfaces_off as usize;
                let size: u32 = buffer
                    .pread_with(curr, endian)
                    .map_err(DexError::Scroll)?;
                curr += 4;
                for _ in 0..size {
                    let type_idx: u16 = buffer
                        .pread_with(curr, endian)
                        .map_err(DexError::Scroll)?;
                    curr += 2;
                    if let Some(itf_name) = resolver.resolve_type(type_idx as u32) {
                        interfaces.push(itf_name);
                    }
                }
            }

            let source_file = if class_def.source_file_idx == NO_INDEX {
                None
            } else {
                resolver.resolve_string(class_def.source_file_idx)
            };

            let mut static_fields = Vec::new();
            let mut instance_fields = Vec::new();
            let mut direct_methods = Vec::new();
            let mut virtual_methods = Vec::new();

            if class_def.class_data_off != 0 {
                let data = class_data::parse(
                    buffer,
                    class_def.class_data_off as usize,
                    header.method_ids_off,
                    protos,
                    resolver,
                    endian,
                )?;
                static_fields = data.static_fields;
                instance_fields = data.instance_fields;
                direct_methods = data.direct_methods;
                virtual_methods = data.virtual_methods;
            }

            Ok(Class {
                name,
                access_flags: class_def.access_flags,
                access_flags_text: translate_access_flags(class_def.access_flags, false),
                superclass,
                interfaces,
                source_file_idx: if class_def.source_file_idx == NO_INDEX {
                    -1
                } else {
                    class_def.source_file_idx as i32
                },
                source_file,
                static_fields,
                instance_fields,
                direct_methods,
                virtual_methods,
            })
        })
        .collect()
}
