mod field_linker;
mod method_linker;

use crate::dex::error::DexError;
use crate::dex::models::{Class, header::RawHeader, raw::RawClassDef, Field};
use crate::dex::models::raw::RawMethodId;
use crate::dex::readers::DexReader;
use crate::dex::linker::DexLinker;
use crate::dex::parsers::{class_data, traits::DexResolver, encoded_value, annotations};
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;
use scroll::Endian;

pub fn parse_linked<'a, R: DexResolver<'a> + Sync + Send>(
    buffer: &'a [u8],
    _header: &RawHeader,
    strings: &[&'a str],
    types: &[&'a str],
    fields: &[Field<'a>],
    methods_sigs: &[String],
    raw_classes: &[RawClassDef],
    raw_method_ids: &[RawMethodId],
    resolver: &R,
    endian: Endian,
) -> Result<Vec<Class<'a>>, DexError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        raw_classes.into_par_iter()
            .map(|raw| {
                link_single_class(buffer, raw, strings, types, fields, methods_sigs, raw_method_ids, resolver, endian)
            })
            .collect()
    }

    #[cfg(target_arch = "wasm32")]
    {
        raw_classes.iter()
            .map(|raw| {
                link_single_class(buffer, raw, strings, types, fields, methods_sigs, raw_method_ids, resolver, endian)
            })
            .collect()
    }
}

fn link_single_class<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    raw: &RawClassDef,
    strings: &[&'a str],
    types: &[&'a str],
    fields: &[Field<'a>],
    methods_sigs: &[String],
    raw_method_ids: &[RawMethodId],
    resolver: &R,
    endian: Endian,
) -> Result<Class<'a>, DexError> {
    let mut class = DexLinker::link_class(raw, strings, types);

    if raw.interfaces_off != 0 {
        let mut reader = DexReader::new(buffer, endian);
        reader.seek(raw.interfaces_off as usize)?;
        let size = reader.read_u32()?;
        for _ in 0..size {
            let type_idx = reader.read_u16()?;
            if let Some(itf) = types.get(type_idx as usize) {
                class.interfaces.push(itf);
            }
        }
    }

    if raw.class_data_off != 0 {
        let mut reader = DexReader::new(buffer, endian);
        let raw_data = class_data::ClassDataParser::parse(&mut reader, raw.class_data_off)?;

        class.static_fields = field_linker::link_fields(&raw_data.static_fields, fields);
        class.instance_fields = field_linker::link_fields(&raw_data.instance_fields, fields);
        class.direct_methods = method_linker::link_methods(buffer, &raw_data.direct_methods, raw_method_ids, strings, methods_sigs, resolver, endian)?;
        class.virtual_methods = method_linker::link_methods(buffer, &raw_data.virtual_methods, raw_method_ids, strings, methods_sigs, resolver, endian)?;
    }

    if raw.static_values_off != 0 {
        let mut reader = DexReader::new(buffer, endian);
        reader.seek(raw.static_values_off as usize)?;
        class.static_values = encoded_value::parse_encoded_array(&mut reader, resolver)?;
    }

    if raw.annotations_off != 0 {
        class.annotations = Some(annotations::parse(buffer, raw.annotations_off as usize, resolver, endian)?);
    }

    Ok(class)
}
