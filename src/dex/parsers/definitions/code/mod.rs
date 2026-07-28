use crate::dex::core::constants::sizes::CODE_ITEM_HEADER;
use crate::dex::error::DexError;
use crate::dex::core::instructions::decoder::InstructionDecoder;
use crate::dex::core::models::{raw::RawCodeItem, Code};
use crate::dex::parsers::core::traits::DexResolver;
use crate::dex::parsers::definitions::debug_info;
use scroll::Pread;

pub mod catches;

pub struct CodeParser;

impl CodeParser {
    pub fn parse<'a, R: DexResolver<'a>>(
        reader: &mut crate::dex::readers::DexReader<'a>,
        offset: usize,
        resolver: &R,
    ) -> Result<Code<'a>, DexError> {
        parse_code_item(reader, offset, resolver)
    }
}

pub fn parse_code_item<'a, R: DexResolver<'a>>(
    reader: &mut crate::dex::readers::DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<Code<'a>, DexError> {
    reader.seek(offset)?;
    let header: RawCodeItem = reader.read_bytes(CODE_ITEM_HEADER)?.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;

    let mut instructions = Vec::new();
    let insns_start = offset + CODE_ITEM_HEADER;
    let decoder = InstructionDecoder::new(resolver);

    let mut pc = 0;
    while pc < (header.insns_size as usize * 2) {
        let (ins, len) = decoder.decode(reader.buffer(), insns_start + pc, insns_start, reader.endian());
        // We need to mark instructions as used.
        let _ = reader.read_bytes(len)?;
        instructions.push(ins);
        pc += len;
    }

    let catches = if header.tries_size > 0 {
        catches::parse_tries(reader, offset, header.tries_size, resolver)?
    } else {
        Vec::new()
    };

    let debug_info = if header.debug_info_off != 0 {
        Some(debug_info::parse_debug_info(reader, header.debug_info_off as usize, resolver)?)
    } else {
        None
    };

    Ok(Code {
        registers_size: header.registers_size,
        ins_size: header.ins_size,
        outs_size: header.outs_size,
        insns_size: header.insns_size,
        instructions,
        catches,
        debug_info,
    })
}
