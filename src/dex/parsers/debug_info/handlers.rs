use crate::dex::error::DexError;
use crate::dex::models::{DebugEntry};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::dex::constants::debug;

pub fn handle_opcode<'a, R: DexResolver<'a>>(
    opcode: u8,
    reader: &mut DexReader<'a>,
    resolver: &R,
    current_address: &mut u32,
    current_line: &mut u32,
    entries: &mut Vec<DebugEntry<'a>>,
) -> Result<bool, DexError> {
    match opcode {
        debug::END_SEQUENCE => return Ok(false),
        debug::ADVANCE_PC => {
            let addr_diff = reader.read_uleb128()?;
            *current_address += addr_diff as u32;
        }
        debug::ADVANCE_LINE => {
            let line_diff = reader.read_sleb128()?;
            *current_line = (*current_line as i32 + line_diff as i32) as u32;
        }
        debug::START_LOCAL => {
            let _reg = reader.read_uleb128()?;
            let name_idx = reader.read_uleb128()?;
            let type_idx = reader.read_uleb128()?;
            entries.push(DebugEntry::StartLocal {
                address_diff: *current_address,
                name: resolver.resolve_string(name_idx as u32).unwrap_or_default(),
                type_name: resolver.resolve_type(type_idx as u32).unwrap_or_default(),
            });
        }
        debug::START_LOCAL_EXTENDED => {
            let _reg = reader.read_uleb128()?;
            let name_idx = reader.read_uleb128()?;
            let type_idx = reader.read_uleb128()?;
            let _sig_idx = reader.read_uleb128()?;
            entries.push(DebugEntry::StartLocal {
                address_diff: *current_address,
                name: resolver.resolve_string(name_idx as u32).unwrap_or_default(),
                type_name: resolver.resolve_type(type_idx as u32).unwrap_or_default(),
            });
        }
        debug::END_LOCAL => {
            let _reg = reader.read_uleb128()?;
            entries.push(DebugEntry::EndLocal {
                address_diff: *current_address,
            });
        }
        debug::RESTART_LOCAL => {
            let _reg = reader.read_uleb128()?;
            entries.push(DebugEntry::RestartLocal {
                address_diff: *current_address,
            });
        }
        debug::SET_PROLOGUE_END | debug::SET_EPILOGUE_BEGIN => {
        }
        debug::SET_FILE => {
            let _name_idx = reader.read_uleb128()?;
        }
        _ => {
            // Special opcodes
            let adjusted_opcode = opcode - debug::FIRST_SPECIAL;
            *current_address += (adjusted_opcode / 15) as u32;
            *current_line = (*current_line as i32 + (adjusted_opcode % 15) as i32 - 4) as u32;
            entries.push(DebugEntry::LineNumber {
                address_diff: *current_address,
                line_diff: *current_line as i32,
            });
        }
    }
    Ok(true)
}
