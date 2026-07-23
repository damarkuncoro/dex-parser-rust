pub mod groups;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IndexType {
    None,
    String,
    Type,
    Method,
    Field,
}

pub struct OpcodeInfo {
    pub name: String,
    pub format: String,
    pub length: usize,
    pub index_type: IndexType,
}

pub struct OpcodeTable;

impl OpcodeTable {
    pub fn get(opcode: u8) -> OpcodeInfo {
        let data = groups::get_basics(opcode)
            .or_else(|| groups::get_moves(opcode))
            .or_else(|| groups::get_consts(opcode))
            .or_else(|| groups::get_flow(opcode))
            .or_else(|| groups::get_ifs(opcode))
            .or_else(|| groups::get_objects(opcode))
            .or_else(|| groups::get_fields(opcode))
            .or_else(|| groups::get_invokes(opcode))
            .or_else(|| groups::get_arithmetic(opcode))
            .unwrap_or(("unknown", "...", 1, IndexType::None));

        OpcodeInfo {
            name: if data.0 == "unknown" {
                format!("op_{:02x}", opcode)
            } else {
                data.0.to_string()
            },
            format: data.1.to_string(),
            length: data.2,
            index_type: data.3,
        }
    }
}
