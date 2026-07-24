use once_cell::sync::Lazy;

pub mod groups;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IndexType {
    None,
    String,
    Type,
    Method,
    Field,
}

#[derive(Clone, Debug)]
pub struct OpcodeInfo {
    pub name: String,
    pub format: String,
    pub length: usize,
    pub index_type: IndexType,
}

static OPCODE_TABLE: Lazy<Vec<OpcodeInfo>> = Lazy::new(|| {
    let mut table = Vec::with_capacity(256);
    for opcode in 0..=255 {
        let data = groups::get_opcode_data(opcode as u8)
            .unwrap_or(("unknown", "...", 1, IndexType::None));

        table.push(OpcodeInfo {
            name: if data.0 == "unknown" {
                format!("op_{:02x}", opcode)
            } else {
                data.0.to_string()
            },
            format: data.1.to_string(),
            length: data.2,
            index_type: data.3,
        });
    }
    table
});

pub struct OpcodeTable;

impl OpcodeTable {
    pub fn get(opcode: u8) -> OpcodeInfo {
        OPCODE_TABLE[opcode as usize].clone()
    }
}
