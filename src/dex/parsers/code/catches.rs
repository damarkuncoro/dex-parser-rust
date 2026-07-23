use crate::dex::error::DexError;
use crate::dex::models::{CatchHandler, TryHandler};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::leb128;
use scroll::{Endian, Pread};

pub fn parse_catches<'a, R: DexResolver<'a>>(
    buffer: &[u8],
    insns_end: usize,
    insns_size: u32,
    tries_size: u16,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<CatchHandler<'a>>, DexError> {
    let mut catches = Vec::new();
    if tries_size > 0 {
        let mut try_offset = insns_end;
        if !insns_size.is_multiple_of(2) {
            try_offset += 2;
        }

        let handler_base_offset = try_offset + (tries_size as usize * 8);

        for i in 0..tries_size {
            let try_item_off = try_offset + (i as usize * 8);
            let start_addr: u32 = buffer.pread_with(try_item_off, endian).map_err(DexError::ScrollError)?;
            let insn_count: u16 = buffer.pread_with(try_item_off + 4, endian).map_err(DexError::ScrollError)?;
            let handler_off: u16 = buffer.pread_with(try_item_off + 6, endian).map_err(DexError::ScrollError)?;

            let mut h_curr = handler_base_offset + handler_off as usize;
            let (size, b) = leb128::read_sleb128(buffer, h_curr)?;
            h_curr += b;

            let mut handlers = Vec::new();
            let abs_size = size.unsigned_abs() as usize;

            for _ in 0..abs_size {
                let (type_idx, b1) = leb128::read_uleb128(buffer, h_curr)?;
                h_curr += b1;
                let (addr, b2) = leb128::read_uleb128(buffer, h_curr)?;
                h_curr += b2;
                handlers.push(TryHandler {
                    type_name: resolver.resolve_type(type_idx as u32).unwrap_or_default(),
                    addr: addr as u32,
                });
            }

            if size <= 0 {
                let (addr, _) = leb128::read_uleb128(buffer, h_curr)?;
                handlers.push(TryHandler {
                    type_name: "Ljava/lang/Throwable;",
                    addr: addr as u32,
                });
            }

            catches.push(CatchHandler {
                start_addr,
                end_addr: start_addr + insn_count as u32,
                handlers,
            });
        }
    }
    Ok(catches)
}
