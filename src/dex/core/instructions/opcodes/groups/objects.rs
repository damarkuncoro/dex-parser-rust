use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x1d => Some(("monitor-enter", "vAA", 1, IndexType::None)),
        0x1e => Some(("monitor-exit", "vAA", 1, IndexType::None)),
        0x1f => Some(("check-cast", "vAA, type@", 2, IndexType::Type)),
        0x20 => Some(("instance-of", "vA, vB, type@", 2, IndexType::Type)),
        0x21 => Some(("array-length", "vA, vB", 1, IndexType::None)),
        0x22 => Some(("new-instance", "vAA, type@", 2, IndexType::Type)),
        0x23 => Some(("new-array", "vA, vB, type@", 2, IndexType::Type)),
        0x24 => Some(("filled-new-array", "{vC..vG}, type@", 3, IndexType::Type)),
        0x25 => Some(("filled-new-array/range", "{vCCCC..vNNNN}, type@", 3, IndexType::Type)),
        0x26 => Some(("fill-array-data", "vAA, +BBBBBBBB", 3, IndexType::None)),

        // AGET
        0x44 => Some(("aget", "vAA, vBB, vCC", 2, IndexType::None)),
        0x45 => Some(("aget-wide", "vAA, vBB, vCC", 2, IndexType::None)),
        0x46 => Some(("aget-object", "vAA, vBB, vCC", 2, IndexType::None)),
        0x47 => Some(("aget-boolean", "vAA, vBB, vCC", 2, IndexType::None)),
        0x48 => Some(("aget-byte", "vAA, vBB, vCC", 2, IndexType::None)),
        0x49 => Some(("aget-char", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4a => Some(("aget-short", "vAA, vBB, vCC", 2, IndexType::None)),

        // APUT
        0x4b => Some(("aput", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4c => Some(("aput-wide", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4d => Some(("aput-object", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4e => Some(("aput-boolean", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4f => Some(("aput-byte", "vAA, vBB, vCC", 2, IndexType::None)),
        0x50 => Some(("aput-char", "vAA, vBB, vCC", 2, IndexType::None)),
        0x51 => Some(("aput-short", "vAA, vBB, vCC", 2, IndexType::None)),
        _ => None,
    }
}
