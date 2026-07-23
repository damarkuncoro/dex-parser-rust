use scroll::{Pread, Endian};
use crate::dex::error::DexError;
use crate::dex::utils::read_uleb128;
use crate::dex::utils::access_flags::translate_access_flags;
use crate::dex::models::{EncodedMethod, Proto, raw::RawMethodId, EncodedField};
use crate::dex::parsers::code;
use crate::dex::parsers::traits::DexResolver;
use crate::dex::constants::sizes::METHOD_ID_ITEM;

pub struct ClassData {
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

pub fn parse<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    header_method_ids_off: u32,
    protos: &[Proto],
    resolver: &R,
    endian: Endian
) -> Result<ClassData, DexError> {
    let mut curr = offset;

    let (static_fields_size, b) = read_uleb128(buffer, curr); curr += b;
    let (instance_fields_size, b) = read_uleb128(buffer, curr); curr += b;
    let (direct_methods_size, b) = read_uleb128(buffer, curr); curr += b;
    let (virtual_methods_size, b) = read_uleb128(buffer, curr); curr += b;

    let static_fields = parse_encoded_fields(buffer, &mut curr, static_fields_size as usize, resolver)?;
    let instance_fields = parse_encoded_fields(buffer, &mut curr, instance_fields_size as usize, resolver)?;

    let direct_methods = parse_encoded_methods(buffer, &mut curr, direct_methods_size as usize, header_method_ids_off, protos, resolver, endian)?;
    let virtual_methods = parse_encoded_methods(buffer, &mut curr, virtual_methods_size as usize, header_method_ids_off, protos, resolver, endian)?;

    Ok(ClassData {
        static_fields,
        instance_fields,
        direct_methods,
        virtual_methods
    })
}

fn parse_encoded_fields<R: DexResolver>(
    buffer: &[u8],
    curr: &mut usize,
    size: usize,
    resolver: &R
) -> Result<Vec<EncodedField>, DexError> {
    let mut encoded_fields = Vec::with_capacity(size);
    let mut last_idx = 0u64;

    for _ in 0..size {
        let (idx_diff, b1) = read_uleb128(buffer, *curr); *curr += b1;
        let (access_flags, b2) = read_uleb128(buffer, *curr); *curr += b2;

        let field_idx = last_idx + idx_diff;
        last_idx = field_idx;

        if let Some(field) = resolver.resolve_field(field_idx as u32) {
            encoded_fields.push(EncodedField {
                name: field.name.clone(),
                type_name: field.type_name.clone(),
                access_flags: access_flags as u32,
                access_flags_text: translate_access_flags(access_flags as u32, false),
            });
        }
    }
    Ok(encoded_fields)
}

fn parse_encoded_methods<R: DexResolver>(
    buffer: &[u8],
    curr: &mut usize,
    size: usize,
    header_method_ids_off: u32,
    protos: &[Proto],
    resolver: &R,
    endian: Endian
) -> Result<Vec<EncodedMethod>, DexError> {
    let mut methods = Vec::with_capacity(size);
    let mut last_idx = 0u64;

    for _ in 0..size {
        let (idx_diff, b1) = read_uleb128(buffer, *curr); *curr += b1;
        let (access_flags, b2) = read_uleb128(buffer, *curr); *curr += b2;
        let (code_off, b3) = read_uleb128(buffer, *curr); *curr += b3;

        let method_idx = last_idx + idx_diff;
        last_idx = method_idx;

        let mid_off = (header_method_ids_off as usize) + (method_idx as usize * METHOD_ID_ITEM);
        let raw_mid: RawMethodId = buffer.pread_with(mid_off, endian)?;

        let name = resolver.resolve_string(raw_mid.name_idx).unwrap_or_default();
        let proto = &protos[raw_mid.proto_idx as usize];

        let signature = format!("({}){}", proto.parameters.join(""), proto.return_type);

        let code = if code_off != 0 {
            code::parse(buffer, code_off as usize, resolver, endian).ok()
        } else {
            None
        };

        methods.push(EncodedMethod {
            name,
            signature,
            access_flags: access_flags as u32,
            access_flags_text: translate_access_flags(access_flags as u32, true),
            code_off,
            code,
        });
    }
    Ok(methods)
}
