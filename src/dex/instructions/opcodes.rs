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

/// Singleton table for Dalvik Opcodes (Modular & Data-Driven)
pub struct OpcodeTable;

impl OpcodeTable {
    pub fn get(opcode: u8) -> OpcodeInfo {
        let (name, format, length, index_type) = match opcode {
            0x00 => ("nop", "", 1, IndexType::None),
            0x01 => ("move", "vA, vB", 1, IndexType::None),
            0x02 => ("move/from16", "vAA, vBBBB", 2, IndexType::None),
            0x03 => ("move/16", "vAAAA, vBBBB", 3, IndexType::None),
            0x04 => ("move/wide", "vA, vB", 1, IndexType::None),
            0x07 => ("move-object", "vA, vB", 1, IndexType::None),
            0x0a => ("move-result", "vAA", 1, IndexType::None),
            0x0c => ("move-result-object", "vAA", 1, IndexType::None),
            0x0d => ("move-exception", "vAA", 1, IndexType::None),
            0x0e => ("return-void", "", 1, IndexType::None),
            0x0f => ("return", "vAA", 1, IndexType::None),
            0x10 => ("return-wide", "vAA", 1, IndexType::None),
            0x11 => ("return-object", "vAA", 1, IndexType::None),
            0x12 => ("const/4", "vA, #+B", 1, IndexType::None),
            0x13 => ("const/16", "vAA, #+BBBB", 2, IndexType::None),
            0x14 => ("const", "vAA, #+BBBBBBBB", 3, IndexType::None),

            // Strings
            0x1a => ("const-string", "vAA, string@", 2, IndexType::String),
            0x1b => ("const-string/jumbo", "vAA, string@", 3, IndexType::String),

            // Types
            0x1c => ("const-class", "vAA, type@", 2, IndexType::Type),
            0x1f => ("check-cast", "vAA, type@", 2, IndexType::Type),
            0x22 => ("new-instance", "vAA, type@", 2, IndexType::Type),
            0x23 => ("new-array", "vA, vB, type@", 2, IndexType::Type),

            0x21 => ("array-length", "vA, vB", 1, IndexType::None),
            0x26 => ("fill-array-data", "vAA, +BBBBBBBB", 3, IndexType::None),
            0x27 => ("throw", "vAA", 1, IndexType::None),
            0x28 => ("goto", "+AA", 1, IndexType::None),
            0x32 => ("if-eq", "vA, vB, +CCCC", 2, IndexType::None),
            0x38 => ("if-eqz", "vA, +BBBB", 2, IndexType::None),

            // Fields
            0x52 => ("iget", "vA, vB, field@", 2, IndexType::Field),
            0x54 => ("iget-object", "vA, vB, field@", 2, IndexType::Field),
            0x59 => ("iput", "vA, vB, field@", 2, IndexType::Field),
            0x5b => ("iput-object", "vA, vB, field@", 2, IndexType::Field),
            0x62 => ("sget", "vAA, field@", 2, IndexType::Field),
            0x67 => ("sput", "vAA, field@", 2, IndexType::Field),

            // Methods
            0x6e => ("invoke-virtual", "{vC..vG}, meth@", 3, IndexType::Method),
            0x6f => ("invoke-super", "{vC..vG}, meth@", 3, IndexType::Method),
            0x70 => ("invoke-direct", "{vC..vG}, meth@", 3, IndexType::Method),
            0x71 => ("invoke-static", "{vC..vG}, meth@", 3, IndexType::Method),
            0x72 => ("invoke-interface", "{vC..vG}, meth@", 3, IndexType::Method),
            0x74 => ("invoke-virtual/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method),

            0x90 => ("add-int", "vAA, vBB, vCC", 2, IndexType::None),
            0xd0 => ("add-int/2addr", "vA, vB", 1, IndexType::None),
            0xd8 => ("add-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
            _ => ("unknown", "...", 1, IndexType::None),
        };

        if name == "unknown" {
            OpcodeInfo {
                name: format!("op_{:02x}", opcode),
                format: "...".to_string(),
                length: 1,
                index_type: IndexType::None,
            }
        } else {
            OpcodeInfo {
                name: name.to_string(),
                format: format.to_string(),
                length,
                index_type,
            }
        }
    }
}
