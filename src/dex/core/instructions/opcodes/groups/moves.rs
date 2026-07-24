use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x01 => Some(("move", "vA, vB", 1, IndexType::None)),
        0x02 => Some(("move/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x03 => Some(("move/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x04 => Some(("move-wide", "vA, vB", 1, IndexType::None)),
        0x05 => Some(("move-wide/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x06 => Some(("move-wide/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x07 => Some(("move-object", "vA, vB", 1, IndexType::None)),
        0x08 => Some(("move-object/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x09 => Some(("move-object/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x0a => Some(("move-result", "vAA", 1, IndexType::None)),
        0x0b => Some(("move-result-wide", "vAA", 1, IndexType::None)),
        0x0c => Some(("move-result-object", "vAA", 1, IndexType::None)),
        0x0d => Some(("move-exception", "vAA", 1, IndexType::None)),
        _ => None,
    }
}
