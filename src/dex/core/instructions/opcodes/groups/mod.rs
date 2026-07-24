pub mod basic;
pub mod moves;
pub mod consts;
pub mod flow;
pub mod ifs;
pub mod objects;
pub mod fields;
pub mod invokes;
pub mod arithmetic;

use super::IndexType;

pub type OpcodeData = (&'static str, &'static str, usize, IndexType);

pub fn get_opcode_data(opcode: u8) -> Option<OpcodeData> {
    basic::get(opcode)
        .or_else(|| moves::get(opcode))
        .or_else(|| consts::get(opcode))
        .or_else(|| flow::get(opcode))
        .or_else(|| ifs::get(opcode))
        .or_else(|| objects::get(opcode))
        .or_else(|| fields::get(opcode))
        .or_else(|| invokes::get(opcode))
        .or_else(|| arithmetic::get(opcode))
}
