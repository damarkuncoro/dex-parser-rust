use scroll::{Endian, Pread};
use crate::dex::error::DexError;
use crate::dex::models::{DebugInfo, DebugEntry};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::utils::read_uleb128;

pub fn parse<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    _endian: Endian,
) -> Result<DebugInfo, DexError> {
    let mut curr = offset;

    let (line_start, b) = read_uleb128(buffer, curr)?; curr += b;
    let (parameters_size, b) = read_uleb128(buffer, curr)?; curr += b;

    let mut parameters = Vec::with_capacity(parameters_size as usize);
    for _ in 0..parameters_size {
        let (name_idx, b) = read_uleb128(buffer, curr)?; curr += b;
        if name_idx == 0 {
            parameters.push(None);
        } else {
            parameters.push(resolver.resolve_string((name_idx - 1) as u32));
        }
    }

    let mut entries = Vec::new();
    loop {
        let opcode: u8 = buffer.pread(*curr).map_err(DexError::ScrollError)?;
        curr += 1;

        match opcode {
            0x00 => break, // DBG_END_SEQUENCE
            0x01 => { // DBG_ADVANCE_PC
                let (addr_diff, b) = read_uleb128(buffer, curr)?; curr += b;
                entries.push(DebugEntry::LineNumber { address_diff: addr_diff as u32, line_diff: 0 });
            }
            0x02 => { // DBG_ADVANCE_LINE
                // Simplified
                let (_, b) = read_uleb128(buffer, curr)?; curr += b;
            }
            0x03 => { // DBG_START_LOCAL
                let (_reg, b) = read_uleb128(buffer, curr)?; curr += b;
                let (name_idx, b) = read_uleb128(buffer, curr)?; curr += b;
                let (type_idx, b) = read_uleb128(buffer, curr)?; curr += b;
                entries.push(DebugEntry::StartLocal {
                    address_diff: 0,
                    name: resolver.resolve_string(name_idx as u32).unwrap_or_default(),
                    type_name: resolver.resolve_type(type_idx as u32).unwrap_or_default(),
                });
            }
            0x05 => { // DBG_END_LOCAL
                let (_reg, b) = read_uleb128(buffer, curr)?; curr += b;
                entries.push(DebugEntry::EndLocal { address_diff: 0 });
            }
            _ => {
                // Simplified: Special opcodes
            }
        }
        if curr >= buffer.len() { break; }
    }

    Ok(DebugInfo {
        line_start: line_start as u32,
        parameters,
        entries,
    })
}
