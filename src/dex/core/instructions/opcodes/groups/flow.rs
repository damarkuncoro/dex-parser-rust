use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x0e => Some(("return-void", "", 1, IndexType::None)),
        0x0f => Some(("return", "vAA", 1, IndexType::None)),
        0x10 => Some(("return-wide", "vAA", 1, IndexType::None)),
        0x11 => Some(("return-object", "vAA", 1, IndexType::None)),
        0x27 => Some(("throw", "vAA", 1, IndexType::None)),
        0x28 => Some(("goto", "+AA", 1, IndexType::None)),
        0x29 => Some(("goto/16", "+AAAA", 2, IndexType::None)),
        0x2a => Some(("goto/32", "+AAAAAAAA", 3, IndexType::None)),
        0x2b => Some(("packed-switch", "vAA, +BBBBBBBB", 3, IndexType::None)),
        0x2c => Some(("sparse-switch", "vAA, +BBBBBBBB", 3, IndexType::None)),
        0x73 => Some(("return-void-no-barrier", "", 1, IndexType::None)),
        _ => None,
    }
}
