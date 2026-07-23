pub mod helpers;

use crate::dex::models::{EncodedValue};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::constants::value_types;
use self::helpers::{read_int, read_uint};
use crate::dex::error::DexError;

pub fn resolve_complex<'a, R: DexResolver<'a>>(
    value_type: u8,
    val: u64,
    resolver: &R,
) -> Result<EncodedValue<'a>, DexError> {
    match value_type {
        value_types::METHOD_TYPE => Ok(EncodedValue::MethodType(val as u32)),
        value_types::METHOD_HANDLE => Ok(EncodedValue::MethodHandle(val as u32)),
        value_types::STRING => Ok(EncodedValue::String(resolver.resolve_string(val as u32).unwrap_or_default())),
        value_types::TYPE => Ok(EncodedValue::Type(resolver.resolve_type(val as u32).unwrap_or_default())),
        value_types::FIELD => {
            let field = resolver.resolve_field(val as u32);
            let name = field.map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name)).unwrap_or_default();
            Ok(EncodedValue::Field(name))
        }
        value_types::METHOD => Ok(EncodedValue::Method(resolver.resolve_method(val as u32).unwrap_or_default())),
        value_types::ENUM => {
            let field = resolver.resolve_field(val as u32);
            let name = field.map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name)).unwrap_or_default();
            Ok(EncodedValue::Enum(name))
        }
        _ => Err(DexError::InvalidIndex(format!("Invalid complex type: 0x{:02x}", value_type))),
    }
}

pub fn parse_primitive<'a>(
    value_type: u8,
    bytes: &[u8],
    value_arg: usize,
) -> Result<EncodedValue<'a>, DexError> {
    let mut pos = 0;
    match value_type {
        value_types::SHORT => {
            let val = read_int(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Short(val as i16))
        }
        value_types::CHAR => {
            let val = read_uint(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Char(val as u16))
        }
        value_types::INT => {
            let val = read_int(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Int(val as i32))
        }
        value_types::LONG => {
            let val = read_int(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Long(val))
        }
        value_types::FLOAT => {
            let val = read_uint(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Float(f32::from_bits(val as u32)))
        }
        value_types::DOUBLE => {
            let val = read_uint(bytes, &mut pos, value_arg + 1)?;
            Ok(EncodedValue::Double(f64::from_bits(val)))
        }
        _ => Err(DexError::InvalidIndex(format!("Invalid primitive type: 0x{:02x}", value_type))),
    }
}
