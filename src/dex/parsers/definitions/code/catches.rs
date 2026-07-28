use crate::dex::core::models::{CatchHandler, TryHandler};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::core::constants::sizes::CODE_ITEM_HEADER;
use crate::trace_parse;
use scroll::Pread;

pub fn parse_tries<'a, R: DexResolver<'a>>(
    reader: &mut crate::dex::readers::DexReader,
    code_off: usize,
    tries_size: u16,
    resolver: &R,
) -> Result<Vec<CatchHandler<'a>>, crate::dex::error::DexError> {
    let mut handlers = Vec::new();
    let endian = reader.endian();
    let buffer = reader.buffer();

    // The reader position might be at the end of instructions.
    // Tries start after instructions, possibly with 2 bytes of padding.
    let insns_size: u32 = buffer.pread_with(code_off + 12, endian).map_err(crate::dex::error::DexError::ScrollError)?;
    let mut tries_start = code_off + CODE_ITEM_HEADER + (insns_size as usize * 2);
    if tries_size > 0 && (tries_start % 4) != 0 {
        // If we have tries, they must be 4-byte aligned.
        // We should mark the padding if it exists.
        if tries_start + 2 <= buffer.len() {
             reader.seek(tries_start)?;
             let _ = reader.read_u16()?;
        }
        tries_start += 2;
    }
    let handlers_base = tries_start + (tries_size as usize * 8);

    trace_parse!("    [Catches] Parsing {} tries at offset 0x{:x}", tries_size, tries_start);

    for i in 0..tries_size {
        reader.seek(tries_start + (i as usize * 8))?;
        let start_addr = reader.read_u32()?;
        let insn_count = reader.read_u16()?;
        let handler_off = reader.read_u16()?;

        let mut catch_handlers = Vec::new();
        reader.seek(handlers_base + handler_off as usize)?;

        let size = reader.read_sleb128()?;

        trace_parse!("      [Try #{}] Handlers: {}, Start: 0x{:x}", i, size, start_addr);

        for _ in 0..size.abs() {
            let type_idx = reader.read_uleb128()?;
            let addr = reader.read_uleb128()?;

            catch_handlers.push(TryHandler {
                type_name: resolver.resolve_type(type_idx as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_else(|| "unknown".to_string()),
                addr: addr as u32,
                _marker: std::marker::PhantomData,
            });
        }

        if size <= 0 {
            let addr = reader.read_uleb128()?;
            catch_handlers.push(TryHandler {
                type_name: "Ljava/lang/Throwable;".to_string(),
                addr: addr as u32,
                _marker: std::marker::PhantomData,
            });
        }

        handlers.push(CatchHandler {
            start_addr,
            end_addr: start_addr + insn_count as u32,
            handlers: catch_handlers,
        });
    }

    Ok(handlers)
}
