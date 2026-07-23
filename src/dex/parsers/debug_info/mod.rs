pub mod handlers;

use crate::dex::error::DexError;
use crate::dex::models::{DebugInfo};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;

pub fn parse<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: scroll::Endian,
) -> Result<DebugInfo<'a>, DexError> {
    let mut reader = DexReader::new(buffer, endian);
    reader.seek(offset)?;

    let line_start = reader.read_uleb128()?;
    let parameters_size = reader.read_uleb128()?;

    let mut parameters = Vec::with_capacity(parameters_size as usize);
    for _ in 0..parameters_size {
        let name_idx_plus_1 = reader.read_uleb128()?;
        if name_idx_plus_1 == 0 {
            parameters.push(None);
        } else {
            parameters.push(resolver.resolve_string((name_idx_plus_1 - 1) as u32));
        }
    }

    let mut entries = Vec::new();
    let mut current_address: u32 = 0;
    let mut current_line: u32 = line_start as u32;

    loop {
        let opcode = match reader.read_u8() {
            Ok(op) => op,
            Err(_) => break, // EOF reached
        };

        if !handlers::handle_opcode(
            opcode,
            &mut reader,
            resolver,
            &mut current_address,
            &mut current_line,
            &mut entries,
        )? {
            break;
        }
    }

    Ok(DebugInfo {
        line_start: line_start as u32,
        parameters,
        entries,
    })
}
