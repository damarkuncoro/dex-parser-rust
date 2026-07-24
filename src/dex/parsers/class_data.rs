use crate::dex::error::DexError;
use crate::dex::readers::DexReader;
use crate::trace_parse;

pub struct RawClassData {
    pub static_fields: Vec<RawEncodedField>,
    pub instance_fields: Vec<RawEncodedField>,
    pub direct_methods: Vec<RawEncodedMethod>,
    pub virtual_methods: Vec<RawEncodedMethod>,
}

pub struct RawEncodedField {
    pub field_idx: u32,
    pub access_flags: u32,
}

pub struct RawEncodedMethod {
    pub method_idx: u32,
    pub access_flags: u32,
    pub code_off: u32,
}

pub fn parse_class_data(reader: &mut DexReader, offset: usize) -> Result<RawClassData, DexError> {
    reader.seek(offset)?;

    trace_parse!("[ClassData] Parsing at offset: 0x{:08x}", offset);

    let static_fields_size = reader.read_uleb128()? as usize;
    let instance_fields_size = reader.read_uleb128()? as usize;
    let direct_methods_size = reader.read_uleb128()? as usize;
    let virtual_methods_size = reader.read_uleb128()? as usize;

    trace_parse!("  [ClassData] Static: {}, Instance: {}, Direct: {}, Virtual: {}",
        static_fields_size, instance_fields_size, direct_methods_size, virtual_methods_size);

    let mut static_fields = Vec::with_capacity(static_fields_size);
    let mut last_idx = 0;
    for _i in 0..static_fields_size {
        let diff = reader.read_uleb128()? as u32;
        last_idx += diff;
        let flags = reader.read_uleb128()? as u32;
        trace_parse!("    [Static Field #{}] Offset: 0x{:08x}, Idx: {}", _i, reader.position(), last_idx);
        static_fields.push(RawEncodedField { field_idx: last_idx, access_flags: flags });
    }

    let mut instance_fields = Vec::with_capacity(instance_fields_size);
    last_idx = 0;
    for _i in 0..instance_fields_size {
        let diff = reader.read_uleb128()? as u32;
        last_idx += diff;
        let flags = reader.read_uleb128()? as u32;
        trace_parse!("    [Instance Field #{}] Offset: 0x{:08x}, Idx: {}", _i, reader.position(), last_idx);
        instance_fields.push(RawEncodedField { field_idx: last_idx, access_flags: flags });
    }

    let mut direct_methods = Vec::with_capacity(direct_methods_size);
    last_idx = 0;
    for _i in 0..direct_methods_size {
        let diff = reader.read_uleb128()? as u32;
        last_idx += diff;
        let flags = reader.read_uleb128()? as u32;
        let code_off = reader.read_uleb128()? as u32;
        trace_parse!("    [Direct Method #{}] Offset: 0x{:08x}, Idx: {}, Code: 0x{:x}", _i, reader.position(), last_idx, code_off);
        direct_methods.push(RawEncodedMethod { method_idx: last_idx, access_flags: flags, code_off });
    }

    let mut virtual_methods = Vec::with_capacity(virtual_methods_size);
    last_idx = 0;
    for _i in 0..virtual_methods_size {
        let diff = reader.read_uleb128()? as u32;
        last_idx += diff;
        let flags = reader.read_uleb128()? as u32;
        let code_off = reader.read_uleb128()? as u32;
        trace_parse!("    [Virtual Method #{}] Offset: 0x{:08x}, Idx: {}, Code: 0x{:x}", _i, reader.position(), last_idx, code_off);
        virtual_methods.push(RawEncodedMethod { method_idx: last_idx, access_flags: flags, code_off });
    }

    Ok(RawClassData { static_fields, instance_fields, direct_methods, virtual_methods })
}
