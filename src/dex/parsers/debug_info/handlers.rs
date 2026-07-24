use crate::dex::core::models::{DebugEntry};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::dex::core::constants::debug;

pub fn parse_debug_entries<'a, R: DexResolver<'a>>(
    reader: &mut DexReader,
    resolver: &R,
) -> Result<Vec<DebugEntry<'a>>, crate::dex::error::DexError> {
    let mut entries = Vec::new();

    loop {
        let opcode = reader.read_u8()?;

        match opcode {
            debug::END_SEQUENCE => break,
            debug::ADVANCE_PC => {
                let addr_diff = reader.read_uleb128()? as u32;
                entries.push(DebugEntry::AdvancePc { addr_diff });
            }
            debug::ADVANCE_LINE => {
                let line_diff = reader.read_sleb128()? as i32;
                entries.push(DebugEntry::AdvanceLine { line_diff });
            }
            debug::START_LOCAL => {
                let register_num = reader.read_uleb128()? as u32;
                let name_idx = reader.read_uleb128()? as i32 - 1;
                let type_idx = reader.read_uleb128()? as i32 - 1;

                let name = if name_idx >= 0 {
                    resolver.resolve_string(name_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };

                let type_name = if type_idx >= 0 {
                    resolver.resolve_type(type_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };

                entries.push(DebugEntry::StartLocal { register_num, name, type_name });
            }
            debug::START_LOCAL_EXTENDED => {
                let register_num = reader.read_uleb128()? as u32;
                let name_idx = reader.read_uleb128()? as i32 - 1;
                let type_idx = reader.read_uleb128()? as i32 - 1;
                let sig_idx = reader.read_uleb128()? as i32 - 1;

                let name = if name_idx >= 0 {
                    resolver.resolve_string(name_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };

                let type_name = if type_idx >= 0 {
                    resolver.resolve_type(type_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };

                let signature = if sig_idx >= 0 {
                    resolver.resolve_string(sig_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };

                entries.push(DebugEntry::StartLocalExtended { register_num, name, type_name, signature });
            }
            debug::END_LOCAL => {
                let register_num = reader.read_uleb128()? as u32;
                entries.push(DebugEntry::EndLocal { register_num });
            }
            debug::RESTART_LOCAL => {
                let register_num = reader.read_uleb128()? as u32;
                entries.push(DebugEntry::RestartLocal { register_num });
            }
            debug::SET_FILE => {
                let name_idx = reader.read_uleb128()? as i32 - 1;
                let name = if name_idx >= 0 {
                    resolver.resolve_string(name_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_default()
                } else { String::new() };
                entries.push(DebugEntry::SetFile { name });
            }
            debug::SET_PROLOGUE_END => entries.push(DebugEntry::SetPrologueEnd),
            debug::SET_EPILOGUE_BEGIN => entries.push(DebugEntry::SetEpilogueBegin),
            _ => {
                let adjusted_opcode = opcode - debug::FIRST_SPECIAL;
                let line_diff = debug::LINE_BASE + (adjusted_opcode % debug::LINE_RANGE) as i32;
                let addr_diff = (adjusted_opcode / debug::LINE_RANGE) as u32;
                entries.push(DebugEntry::SpecialOpcode { opcode, line_diff, addr_diff });
            }
        }
    }

    Ok(entries)
}
