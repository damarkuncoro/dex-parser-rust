use crate::dex::core::models::{EncodedValue};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::core::constants::value_types;
use crate::dex::readers::DexReader;

pub mod helpers;
pub use helpers::*;

pub fn parse_encoded_value<'a, R: DexResolver<'a>>(
    reader: &mut DexReader,
    resolver: &R,
) -> Result<EncodedValue<'a>, crate::dex::error::DexError> {
    let header = reader.read_u8()?;
    let val_type = header & 0x1f;
    let arg = (header >> 5) as usize;

    let buffer = reader.buffer();
    let mut curr = reader.position();

    match val_type {
        value_types::BYTE => {
            let val = read_int(buffer, &mut curr, arg + 1)? as i8;
            reader.seek(curr)?;
            Ok(EncodedValue::Byte(val))
        },
        value_types::SHORT => {
            let val = read_int(buffer, &mut curr, arg + 1)? as i16;
            reader.seek(curr)?;
            Ok(EncodedValue::Short(val))
        },
        value_types::CHAR => {
            let val = read_uint(buffer, &mut curr, arg + 1)? as u16;
            reader.seek(curr)?;
            Ok(EncodedValue::Char(val))
        },
        value_types::INT => {
            let val = read_int(buffer, &mut curr, arg + 1)? as i32;
            reader.seek(curr)?;
            Ok(EncodedValue::Int(val))
        },
        value_types::LONG => {
            let val = read_int(buffer, &mut curr, arg + 1)?;
            reader.seek(curr)?;
            Ok(EncodedValue::Long(val))
        },
        value_types::STRING => {
            let val = read_uint(buffer, &mut curr, arg + 1)?;
            reader.seek(curr)?;
            Ok(EncodedValue::String(resolver.resolve_string(val as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()))
        },
        value_types::TYPE => {
            let val = read_uint(buffer, &mut curr, arg + 1)?;
            reader.seek(curr)?;
            Ok(EncodedValue::Type(resolver.resolve_type(val as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()))
        },
        _ => {
            // Re-use main parser for other types to avoid duplication
            reader.seek(reader.position() - 1)?; // Backtrack header
            super::parse_encoded_value(reader, resolver)
        }
    }
}
