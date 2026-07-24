use crate::dex::core::models::{Class, header::RawHeader, raw::RawClassDef, Field};
use crate::dex::core::models::raw::RawMethodId;
use crate::dex::parsers::{class_data, traits::DexResolver, annotations};
use crate::dex::error::DexError;
use rayon::prelude::*;

use crate::dex::core::utils::byte_tracker::ByteTracker;
use std::sync::{Arc};
use parking_lot::Mutex;

pub mod field_linker;
pub mod method_linker;

pub fn parse_linked<'a, R: DexResolver<'a> + Sync + Send>(
    buffer: &'a [u8],
    _header: &RawHeader,
    strings: &[&'a [u8]],
    types: &[&'a [u8]],
    fields: &[Field<'a>],
    methods_display: &[String],
    raw_classes: &[RawClassDef],
    raw_methods: &[RawMethodId],
    resolver: &R,
    endian: scroll::Endian,
    tracker: Arc<Mutex<ByteTracker>>,
) -> Result<Vec<Class<'a>>, DexError> {
    raw_classes.par_iter().map(|raw| {
        let mut class = crate::dex::core::linker::DexLinker::link_class(raw, strings, types);
        let class_name = class.name.clone();
        let tracker = tracker.clone();

        let res: Result<(), DexError> = (|| {
            let mut reader = crate::dex::readers::DexReader::new(buffer, endian).with_tracker(tracker);
            if raw.interfaces_off != 0 {
                reader.seek(raw.interfaces_off as usize)?;
                let size = reader.read_u32()?;
                for _ in 0..size {
                    let type_idx = reader.read_u16()?;
                    if let Some(&itf_bytes) = types.get(type_idx as usize) {
                        class.interfaces.push(format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(itf_bytes)));
                    }
                }
            }

            if raw.class_data_off != 0 {
                let data = class_data::parse_class_data(&mut reader, raw.class_data_off as usize)?;
                class.static_fields = field_linker::link_fields(&data.static_fields, fields, resolver);
                class.instance_fields = field_linker::link_fields(&data.instance_fields, fields, resolver);
                class.direct_methods = method_linker::link_methods(&mut reader, &data.direct_methods, methods_display, raw_methods, resolver)?;
                class.virtual_methods = method_linker::link_methods(&mut reader, &data.virtual_methods, methods_display, raw_methods, resolver)?;
            }

            if raw.annotations_off != 0 {
                class.annotations = Some(annotations::parse_annotations_directory_with_reader(&mut reader, raw.annotations_off as usize, resolver)?);
            }

            if raw.static_values_off != 0 {
                reader.seek(raw.static_values_off as usize)?;
                class.static_values = crate::dex::parsers::encoded_value::parse_encoded_array(&mut reader, resolver)?;
            }
            Ok(())
        })();

        res.map_err(|e| {
            eprintln!("Warning: Skipping class {} due to error: {}", class_name, e);
            e
        }).ok(); // Change from ? to ok() to skip errors

        Ok(class)
    }).collect()
}
