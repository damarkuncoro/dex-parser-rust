use super::super::IndexType;
use super::OpcodeData;

pub fn get(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x00 => Some(("nop", "", 1, IndexType::None)),
        _ => None,
    }
}
