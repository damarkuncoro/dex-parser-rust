use crate::dex::error::DexError;
use crate::dex::readers::DexReader;

pub struct RawEncodedField {
    pub field_idx_diff: u64,
    pub access_flags: u64,
}

pub struct RawEncodedMethod {
    pub method_idx_diff: u64,
    pub access_flags: u64,
    pub code_off: u64,
}

pub struct RawClassData {
    pub static_fields: Vec<RawEncodedField>,
    pub instance_fields: Vec<RawEncodedField>,
    pub direct_methods: Vec<RawEncodedMethod>,
    pub virtual_methods: Vec<RawEncodedMethod>,
}

pub struct ClassDataParser;

impl ClassDataParser {
    pub fn parse(reader: &mut DexReader, offset: u32) -> Result<RawClassData, DexError> {
        if offset == 0 {
            return Ok(RawClassData {
                static_fields: Vec::new(), instance_fields: Vec::new(),
                direct_methods: Vec::new(), virtual_methods: Vec::new(),
            });
        }
        reader.seek(offset as usize)?;

        let static_fields_size = reader.read_uleb128()?;
        let instance_fields_size = reader.read_uleb128()?;
        let direct_methods_size = reader.read_uleb128()?;
        let virtual_methods_size = reader.read_uleb128()?;

        Ok(RawClassData {
            static_fields: Self::read_fields(reader, static_fields_size as usize)?,
            instance_fields: Self::read_fields(reader, instance_fields_size as usize)?,
            direct_methods: Self::read_methods(reader, direct_methods_size as usize)?,
            virtual_methods: Self::read_methods(reader, virtual_methods_size as usize)?,
        })
    }

    fn read_fields(reader: &mut DexReader, size: usize) -> Result<Vec<RawEncodedField>, DexError> {
        let mut fields = Vec::with_capacity(size);
        for _ in 0..size {
            fields.push(RawEncodedField {
                field_idx_diff: reader.read_uleb128()?,
                access_flags: reader.read_uleb128()?,
            });
        }
        Ok(fields)
    }

    fn read_methods(reader: &mut DexReader, size: usize) -> Result<Vec<RawEncodedMethod>, DexError> {
        let mut methods = Vec::with_capacity(size);
        for _ in 0..size {
            methods.push(RawEncodedMethod {
                method_idx_diff: reader.read_uleb128()?,
                access_flags: reader.read_uleb128()?,
                code_off: reader.read_uleb128()?,
            });
        }
        Ok(methods)
    }
}
