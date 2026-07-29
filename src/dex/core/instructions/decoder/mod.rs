use crate::dex::core::instructions::opcodes::{OpcodeTable, groups};
use crate::dex::core::models::Instruction;
use crate::dex::parsers::traits::DexResolver;
use scroll::{Endian, Pread};
use std::marker::PhantomData;

pub mod helpers;

/// Decodes raw Dalvik bytecode into high-level `Instruction` structures.
pub struct InstructionDecoder<'res, 'a, R: DexResolver<'a>> {
    resolver: &'res R,
    _marker: PhantomData<&'a ()>,
}

impl<'res, 'a, R: DexResolver<'a>> InstructionDecoder<'res, 'a, R> {
    pub fn new(resolver: &'res R) -> Self {
        Self {
            resolver,
            _marker: PhantomData,
        }
    }

    pub fn decode(
        &self,
        buffer: &[u8],
        pc: usize,
        curr: usize,
        endian: Endian,
    ) -> (Instruction, usize) {
        let opcode_byte: u8 = buffer.pread_with(pc, endian).unwrap_or(0);
        let info = OpcodeTable::get(opcode_byte);

        let mut units = Vec::new();
        for i in 0..info.length {
            let unit: u16 = buffer.pread_with(pc + (i * 2), endian).unwrap_or(0);
            units.push(unit);
        }

        let mut description = groups::get_opcode_data(opcode_byte)
            .map(|d| d.1.to_string())
            .unwrap_or_else(|| "...".to_string());

        let current_instr_byte_addr = pc - curr;
        let mut struct_index = None;
        let mut struct_resolved = None;
        let mut registers = Vec::new();
        let mut target_offset = None;
        let mut immediates = Vec::new();

        if !units.is_empty() {
            let op_unit = units[0];

            registers = helpers::extract_registers(&description, op_unit, &units);
            target_offset = helpers::extract_branch_target(&description, op_unit, &units)
                .map(|off| (current_instr_byte_addr as i32 + (off * 2)) as u32);
            immediates = helpers::extract_immediates(&description, op_unit, &units, buffer, pc, endian);

            helpers::substitute_special(&mut description, op_unit, &units);
            helpers::substitute_immediates(&mut description, op_unit, &units, buffer, pc, endian);
            helpers::substitute_branches(&mut description, op_unit, &units, current_instr_byte_addr);
            helpers::substitute_registers(&mut description, op_unit, &units);

            if info.index_type != crate::dex::core::instructions::opcodes::IndexType::None {
                let index: u32 = if opcode_byte == 0x1b { // const-string/jumbo
                    buffer.pread_with(pc + 2, endian).unwrap_or(0)
                } else {
                    units.get(1).cloned().unwrap_or(0) as u32
                };
                struct_index = Some(index);
                struct_resolved = Some(helpers::resolve_xref(&mut description, info.index_type, index, self.resolver));
            }
        }

        let instruction = Instruction {
            offset: current_instr_byte_addr,
            opcode: opcode_byte,
            name: info.name,
            description,
            index: struct_index,
            resolved_value: struct_resolved,
            registers,
            target_offset,
            immediates,
        };

        (instruction, info.length * 2)
    }
}
