pub mod types;

use crate::dex::error::DexError;
use crate::dex::models::{EncodedValue, EncodedAnnotation, AnnotationElement};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::dex::constants::value_types;
use self::types::{parse_primitive, resolve_complex};

pub fn parse<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    resolver: &R,
) -> Result<EncodedValue<'a>, DexError> {
    let header = reader.read_u8()?;
    let value_type = header & 0x1f;
    let value_arg = (header >> 5) as usize;

    match value_type {
        value_types::BYTE => Ok(EncodedValue::Byte(reader.read_u8()? as i8)),
        value_types::SHORT | value_types::CHAR | value_types::INT | value_types::LONG | value_types::FLOAT | value_types::DOUBLE => {
            let bytes = reader.read_bytes(value_arg + 1)?;
            parse_primitive(value_type, bytes, value_arg)
        }
        value_types::METHOD_TYPE | value_types::METHOD_HANDLE | value_types::STRING | value_types::TYPE | value_types::FIELD | value_types::METHOD | value_types::ENUM => {
            let bytes = reader.read_bytes(value_arg + 1)?;
            let mut pos = 0;
            let val = types::helpers::read_uint(bytes, &mut pos, value_arg + 1)?;
            resolve_complex(value_type, val, resolver)
        }
        value_types::ARRAY => {
            let size = reader.read_uleb128()?;
            let mut values = Vec::with_capacity(size as usize);
            for _ in 0..size {
                values.push(parse(reader, resolver)?);
            }
            Ok(EncodedValue::Array(values))
        }
        value_types::ANNOTATION => {
            Ok(EncodedValue::Annotation(parse_annotation(reader, resolver)?))
        }
        value_types::NULL => Ok(EncodedValue::Null),
        value_types::BOOLEAN => Ok(EncodedValue::Boolean(value_arg != 0)),
        _ => Err(DexError::InvalidIndex(format!("Unknown encoded value type: 0x{:02x}", value_type))),
    }
}

pub fn parse_annotation<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    resolver: &R,
) -> Result<EncodedAnnotation<'a>, DexError> {
    let type_idx = reader.read_uleb128()?;
    let size = reader.read_uleb128()?;

    let type_name = resolver.resolve_type(type_idx as u32).unwrap_or_default();
    let mut elements = Vec::with_capacity(size as usize);

    for _ in 0..size {
        let name_idx = reader.read_uleb128()?;
        let name = resolver.resolve_string(name_idx as u32).unwrap_or_default();
        let value = parse(reader, resolver)?;
        elements.push(AnnotationElement { name, value });
    }

    Ok(EncodedAnnotation { type_name, elements })
}
