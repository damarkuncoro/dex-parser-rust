use crate::dex::constants::sizes::CODE_ITEM_HEADER;
use crate::dex::error::DexError;
use crate::dex::instructions::decoder::InstructionDecoder;
use crate::dex::models::{raw::RawCodeItem, CatchHandler, Code, TryHandler};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::utils::{read_sleb128, read_uleb128};
use scroll::{Endian, Pread};

pub fn parse<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Code, DexError> {
    let mut curr = offset;
    let raw: RawCodeItem = buffer.pread_with(curr, endian)?;
    curr += CODE_ITEM_HEADER;

    let decoder = InstructionDecoder::new(resolver);
    let mut instructions = Vec::new();
    let insns_size_bytes = raw.insns_size as usize * 2;
    let insns_end = curr + insns_size_bytes;

    let mut pc = curr;
    while pc < insns_end && pc < buffer.len() {
        let (ins, length) = decoder.decode(buffer, pc, curr, endian);
        instructions.push(ins);
        pc += length;
    }

    let catches = parse_catches(
        buffer,
        insns_end,
        raw.insns_size,
        raw.tries_size,
        resolver,
        endian,
    )?;

    Ok(Code {
        registers_size: raw.registers_size,
        ins_size: raw.ins_size,
        outs_size: raw.outs_size,
        insns_size: raw.insns_size,
        instructions,
        catches,
    })
}

fn parse_catches<R: DexResolver>(
    buffer: &[u8],
    insns_end: usize,
    insns_size: u32,
    tries_size: u16,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<CatchHandler>, DexError> {
    let mut catches = Vec::new();
    if tries_size > 0 {
        let mut try_offset = insns_end;
        if !insns_size.is_multiple_of(2) {
            try_offset += 2;
        }

        let handler_base_offset = try_offset + (tries_size as usize * 8);

        for i in 0..tries_size {
            let try_item_off = try_offset + (i as usize * 8);
            let start_addr: u32 = buffer.pread_with(try_item_off, endian)?;
            let insn_count: u16 = buffer.pread_with(try_item_off + 4, endian)?;
            let handler_off: u16 = buffer.pread_with(try_item_off + 6, endian)?;

            let mut h_curr = handler_base_offset + handler_off as usize;
            let (size, b) = read_sleb128(buffer, h_curr);
            h_curr += b;

            let mut handlers = Vec::new();
            let abs_size = size.unsigned_abs() as usize;

            for _ in 0..abs_size {
                let (type_idx, b1) = read_uleb128(buffer, h_curr);
                h_curr += b1;
                let (addr, b2) = read_uleb128(buffer, h_curr);
                h_curr += b2;
                handlers.push(TryHandler {
                    type_name: resolver.resolve_type(type_idx as u32).unwrap_or_default(),
                    addr: addr as u32,
                });
            }

            if size <= 0 {
                let (addr, _) = read_uleb128(buffer, h_curr);
                handlers.push(TryHandler {
                    type_name: "Ljava/lang/Throwable;".to_string(),
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
