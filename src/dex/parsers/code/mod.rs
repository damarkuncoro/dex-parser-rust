pub mod catches;

use crate::dex::constants::sizes::CODE_ITEM_HEADER;
use crate::dex::error::DexError;
use crate::dex::instructions::decoder::InstructionDecoder;
use crate::dex::models::{raw::RawCodeItem, Code};
use crate::dex::parsers::{debug_info, traits::DexResolver};
use scroll::{Endian, Pread};
use self::catches::parse_catches;

pub fn parse<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Code<'a>, DexError> {
    let mut curr = offset;
    let raw: RawCodeItem = buffer.pread_with(curr, endian).map_err(DexError::ScrollError)?;
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

    let dbg_info = if raw.debug_info_off != 0 {
        debug_info::parse(buffer, raw.debug_info_off as usize, resolver, endian).ok()
    } else {
        None
    };

    Ok(Code {
        registers_size: raw.registers_size,
        ins_size: raw.ins_size,
        outs_size: raw.outs_size,
        insns_size: raw.insns_size,
        instructions,
        catches,
        debug_info: dbg_info,
    })
}
