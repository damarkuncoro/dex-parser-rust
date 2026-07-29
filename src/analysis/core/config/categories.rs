use crate::dex::core::instructions::opcodes::OpcodeTable;

/// Opcodes categories for CFG Analysis.
pub struct OpcodeCategories;

impl OpcodeCategories {
    pub fn is_branch(opcode: u8) -> bool {
        OpcodeTable::get(opcode).can_branch
    }

    pub fn is_goto(opcode: u8) -> bool {
        // In AOSP InstrUtils, goto is a type of branch
        OpcodeTable::get(opcode).can_branch && !OpcodeTable::get(opcode).can_continue
    }

    pub fn is_switch(opcode: u8) -> bool {
        OpcodeTable::get(opcode).can_switch
    }

    pub fn is_terminator(opcode: u8) -> bool {
        OpcodeTable::get(opcode).can_return || OpcodeTable::get(opcode).can_throw
    }
}
