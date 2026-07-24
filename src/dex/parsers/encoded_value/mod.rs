use crate::dex::core::models::{EncodedValue, EncodedAnnotation, AnnotationElement};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::dex::core::constants::value_types;
use crate::trace_parse;

pub mod types;

pub fn parse_encoded_value<'a, R: DexResolver<'a>>(
    reader: &mut DexReader,
    resolver: &R,
) -> Result<EncodedValue<'a>, crate::dex::error::DexError> {
    let _start_pos = reader.position();
    let header = reader.read_u8()?;
    let value_type = header & 0x1f;
    let value_arg = (header >> 5) as usize;

    let buffer = reader.buffer();
    let mut curr = reader.position();

    trace_parse!("[EncodedValue] Offset: 0x{:04x}, Type: 0x{:02x}, Arg: {}", _start_pos, value_type, value_arg);

    let value = match value_type {
        value_types::BYTE => {
            let start = reader.position();
            let val = types::read_int(buffer, &mut curr, value_arg + 1)? as i8;
            reader.read_bytes(curr - start)?;
            EncodedValue::Byte(val)
        },
        value_types::SHORT => {
            let start = reader.position();
            let val = types::read_int(buffer, &mut curr, value_arg + 1)? as i16;
            reader.read_bytes(curr - start)?;
            EncodedValue::Short(val)
        },
        value_types::CHAR => {
            let start = reader.position();
            let val = types::read_uint(buffer, &mut curr, value_arg + 1)? as u16;
            reader.read_bytes(curr - start)?;
            EncodedValue::Char(val)
        },
        value_types::INT => {
            let start = reader.position();
            let val = types::read_int(buffer, &mut curr, value_arg + 1)? as i32;
            reader.read_bytes(curr - start)?;
            EncodedValue::Int(val)
        },
        value_types::LONG => {
            let start = reader.position();
            let val = types::read_int(buffer, &mut curr, value_arg + 1)?;
            reader.read_bytes(curr - start)?;
            EncodedValue::Long(val)
        },
        value_types::FLOAT => {
            let start = reader.position();
            let val = types::read_float(buffer, &mut curr, value_arg + 1)?;
            reader.read_bytes(curr - start)?;
            EncodedValue::Float(val)
        },
        value_types::DOUBLE => {
            let start = reader.position();
            let val = types::read_double(buffer, &mut curr, value_arg + 1)?;
            reader.read_bytes(curr - start)?;
            EncodedValue::Double(val)
        },
        value_types::STRING => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::String(resolver.resolve_string(idx).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default())
        },
        value_types::TYPE => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::Type(resolver.resolve_type(idx).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default())
        },
        value_types::FIELD => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::Field(resolver.resolve_field(idx).map(|f| f.name).unwrap_or_default())
        },
        value_types::METHOD => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::Method(resolver.resolve_method(idx).unwrap_or_default())
        },
        value_types::ENUM => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::Enum(resolver.resolve_field(idx).map(|f| f.name).unwrap_or_default())
        },
        value_types::METHOD_TYPE => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::MethodType(idx)
        },
        value_types::METHOD_HANDLE => {
            let start = reader.position();
            let idx = types::read_uint(buffer, &mut curr, value_arg + 1)? as u32;
            reader.read_bytes(curr - start)?;
            EncodedValue::MethodHandle(idx)
        },
        value_types::ARRAY => {
            let size = reader.read_uleb128()? as usize;
            let mut values = Vec::with_capacity(size);
            for _i in 0..size {
                trace_parse!("  [Array] Element {}/{}", _i + 1, size);
                values.push(parse_encoded_value(reader, resolver)?);
            }
            EncodedValue::Array(values)
        },
        value_types::ANNOTATION => {
            EncodedValue::Annotation(parse_encoded_annotation(reader, resolver)?)
        },
        value_types::NULL => EncodedValue::Null,
        value_types::BOOLEAN => EncodedValue::Boolean(value_arg != 0),
        _ => {
            trace_parse!("  [Error] Unsupported or unknown value type: 0x{:02x} at offset 0x{:04x}", value_type, _start_pos);
            return Err(crate::dex::error::DexError::InvalidMagic);
        }
    };

    Ok(value)
}

pub fn parse_encoded_annotation<'a, R: DexResolver<'a>>(
    reader: &mut DexReader,
    resolver: &R,
) -> Result<EncodedAnnotation<'a>, crate::dex::error::DexError> {
    let type_idx = reader.read_uleb128()? as u32;
    let size = reader.read_uleb128()? as usize;
    let type_name = resolver.resolve_type(type_idx).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_else(|| format!("type@{:04x}", type_idx));

    trace_parse!("  [Annotation] Type: {}, Elements: {}", type_name, size);

    let mut elements = Vec::with_capacity(size);
    for _i in 0..size {
        let name_idx = reader.read_uleb128()? as u32;
        let name = resolver.resolve_string(name_idx).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_else(|| format!("string@{:04x}", name_idx));
        trace_parse!("    [Element] {}/{} : {}", _i + 1, size, name);
        let value = parse_encoded_value(reader, resolver)?;
        elements.push(AnnotationElement { name, value });
    }

    Ok(EncodedAnnotation { type_name, elements })
}

pub fn parse_encoded_array<'a, R: DexResolver<'a>>(
    reader: &mut DexReader,
    resolver: &R,
) -> Result<Vec<EncodedValue<'a>>, crate::dex::error::DexError> {
    let size = reader.read_uleb128()? as usize;
    let mut values = Vec::with_capacity(size);
    for _ in 0..size {
        values.push(parse_encoded_value(reader, resolver)?);
    }
    Ok(values)
}
