use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x32 => Some(("if-eq", "vA, vB, +CCCC", 2, IndexType::None)),
        0x33 => Some(("if-ne", "vA, vB, +CCCC", 2, IndexType::None)),
        0x34 => Some(("if-lt", "vA, vB, +CCCC", 2, IndexType::None)),
        0x35 => Some(("if-ge", "vA, vB, +CCCC", 2, IndexType::None)),
        0x36 => Some(("if-gt", "vA, vB, +CCCC", 2, IndexType::None)),
        0x37 => Some(("if-le", "vA, vB, +CCCC", 2, IndexType::None)),
        0x38 => Some(("if-eqz", "vA, +BBBB", 2, IndexType::None)),
        0x39 => Some(("if-nez", "vA, +BBBB", 2, IndexType::None)),
        0x3a => Some(("if-ltz", "vA, +BBBB", 2, IndexType::None)),
        0x3b => Some(("if-gez", "vA, +BBBB", 2, IndexType::None)),
        0x3c => Some(("if-gtz", "vA, +BBBB", 2, IndexType::None)),
        0x3d => Some(("if-lez", "vA, +BBBB", 2, IndexType::None)),
        _ => None,
    }
}
