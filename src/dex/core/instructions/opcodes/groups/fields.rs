use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x52 => Some(("iget", "vA, vB, field@", 2, IndexType::Field)),
        0x53 => Some(("iget-wide", "vA, vB, field@", 2, IndexType::Field)),
        0x54 => Some(("iget-object", "vA, vB, field@", 2, IndexType::Field)),
        0x55 => Some(("iget-boolean", "vA, vB, field@", 2, IndexType::Field)),
        0x56 => Some(("iget-byte", "vA, vB, field@", 2, IndexType::Field)),
        0x57 => Some(("iget-char", "vA, vB, field@", 2, IndexType::Field)),
        0x58 => Some(("iget-short", "vA, vB, field@", 2, IndexType::Field)),
        0x59 => Some(("iput", "vA, vB, field@", 2, IndexType::Field)),
        0x5a => Some(("iput-wide", "vA, vB, field@", 2, IndexType::Field)),
        0x5b => Some(("iput-object", "vA, vB, field@", 2, IndexType::Field)),
        0x5c => Some(("iput-boolean", "vA, vB, field@", 2, IndexType::Field)),
        0x5d => Some(("iput-byte", "vA, vB, field@", 2, IndexType::Field)),
        0x5e => Some(("iput-char", "vA, vB, field@", 2, IndexType::Field)),
        0x5f => Some(("iput-short", "vA, vB, field@", 2, IndexType::Field)),
        0x60 => Some(("sget", "vAA, field@", 2, IndexType::Field)),
        0x61 => Some(("sget-wide", "vAA, field@", 2, IndexType::Field)),
        0x62 => Some(("sget-object", "vAA, field@", 2, IndexType::Field)),
        0x63 => Some(("sget-boolean", "vAA, field@", 2, IndexType::Field)),
        0x64 => Some(("sget-byte", "vAA, field@", 2, IndexType::Field)),
        0x65 => Some(("sget-char", "vAA, field@", 2, IndexType::Field)),
        0x66 => Some(("sget-short", "vAA, field@", 2, IndexType::Field)),
        0x67 => Some(("sput", "vAA, field@", 2, IndexType::Field)),
        0x68 => Some(("sput-wide", "vAA, field@", 2, IndexType::Field)),
        0x69 => Some(("sput-object", "vAA, field@", 2, IndexType::Field)),
        0x6a => Some(("sput-boolean", "vAA, field@", 2, IndexType::Field)),
        0x6b => Some(("sput-byte", "vAA, field@", 2, IndexType::Field)),
        0x6c => Some(("sput-char", "vAA, field@", 2, IndexType::Field)),
        0x6d => Some(("sput-short", "vAA, field@", 2, IndexType::Field)),

        // Advanced/Volatile/Quicken (API level 1+)
        0xe3 => Some(("iget-volatile", "vA, vB, field@", 2, IndexType::Field)),
        0xe4 => Some(("iput-volatile", "vA, vB, field@", 2, IndexType::Field)),
        0xe5 => Some(("sget-volatile", "vAA, field@", 2, IndexType::Field)),
        0xe6 => Some(("sput-volatile", "vAA, field@", 2, IndexType::Field)),
        0xe7 => Some(("iget-object-volatile", "vA, vB, field@", 2, IndexType::Field)),

        0xe8 => Some(("iget-wide-volatile", "vA, vB, field@", 2, IndexType::Field)),
        0xe9 => Some(("iput-wide-volatile", "vA, vB, field@", 2, IndexType::Field)),
        0xea => Some(("sget-wide-volatile", "vAA, field@", 2, IndexType::Field)),
        0xeb => Some(("sput-wide-volatile", "vAA, field@", 2, IndexType::Field)),

        0xf0 => Some(("iput-object-volatile", "vA, vB, field@", 2, IndexType::Field)),
        0xf1 => Some(("sget-object-volatile", "vAA, field@", 2, IndexType::Field)),

        0xf2 => Some(("iget-quick", "vA, vB, offset@", 2, IndexType::None)),
        0xf3 => Some(("iget-wide-quick", "vA, vB, offset@", 2, IndexType::None)),
        0xf4 => Some(("iget-object-quick", "vA, vB, offset@", 2, IndexType::None)),
        0xf5 => Some(("iput-quick", "vA, vB, offset@", 2, IndexType::None)),
        0xf6 => Some(("iput-wide-quick", "vA, vB, offset@", 2, IndexType::None)),
        0xf7 => Some(("iput-object-quick", "vA, vB, offset@", 2, IndexType::None)),
        _ => None,
    }
}
