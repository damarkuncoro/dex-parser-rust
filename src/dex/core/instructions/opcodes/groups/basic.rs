use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x00 => Some(("nop", "", 1, IndexType::None)),

        0x2d => Some(("cmpl-float", "vAA, vBB, vCC", 2, IndexType::None)),
        0x2e => Some(("cmpg-float", "vAA, vBB, vCC", 2, IndexType::None)),
        0x2f => Some(("cmpl-double", "vAA, vBB, vCC", 2, IndexType::None)),
        0x30 => Some(("cmpg-double", "vAA, vBB, vCC", 2, IndexType::None)),
        0x31 => Some(("cmp-long", "vAA, vBB, vCC", 2, IndexType::None)),

        // Misc / Advanced (from API level 8+)
        0xec => Some(("breakpoint", "", 1, IndexType::None)),
        0xed => Some(("throw-verification-error", "vAA, error_idx@", 2, IndexType::None)),
        0xee => Some(("execute-inline", "{vC..vG}, inline_idx@", 3, IndexType::None)),
        0xef => Some(("execute-inline/range", "{vCCCC..vNNNN}, inline_idx@", 3, IndexType::None)),
        _ => None,
    }
}
