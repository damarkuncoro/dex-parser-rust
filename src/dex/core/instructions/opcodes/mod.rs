use once_cell::sync::Lazy;

pub mod groups;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IndexType {
    None,
    String,
    Type,
    Method,
    Field,
    CallSite,
    MethodHandle,
    Proto,
}

#[derive(Clone, Debug)]
pub struct OpcodeInfo {
    pub name: String,
    pub format: String,
    pub length: usize,
    pub index_type: IndexType,
    pub is_terminator: bool,
    pub is_branch: bool,
    pub is_goto: bool,
    pub is_switch: bool,
}

static OPCODE_TABLE: Lazy<Vec<OpcodeInfo>> = Lazy::new(|| {
    let mut table = Vec::with_capacity(256);
    for opcode in 0..=255 {
        let op = opcode as u8;
        let data = groups::get_opcode_data(op)
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
            is_terminator: is_terminator_op(op),
            is_branch: is_branch_op(op),
            is_goto: is_goto_op(op),
            is_switch: is_switch_op(op),
        });
    }
    table
});

fn is_terminator_op(op: u8) -> bool {
    match op {
        0x0e..=0x11 | 0x27 => true, // return*, throw
        _ => false,
    }
}

fn is_branch_op(op: u8) -> bool {
    match op {
        0x32..=0x3d => true, // if-*
        _ => false,
    }
}

fn is_goto_op(op: u8) -> bool {
    match op {
        0x28..=0x2a => true, // goto*
        _ => false,
    }
}

fn is_switch_op(op: u8) -> bool {
    match op {
        0x2b..=0x2c => true, // packed-switch, sparse-switch
        _ => false,
    }
}

pub struct OpcodeTable;

impl OpcodeTable {
    pub fn get(opcode: u8) -> OpcodeInfo {
        OPCODE_TABLE[opcode as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_coverage() {
        let mut unknown_count = 0;
        let mut known_opcodes = Vec::new();

        for i in 0..=255 {
            let info = OpcodeTable::get(i as u8);
            if info.name.starts_with("op_") {
                unknown_count += 1;
            } else {
                known_opcodes.push(format!("0x{:02x}: {}", i, info.name));
            }
        }

        println!("Total Known Opcodes: {}", known_opcodes.len());
        println!("Total Unknown/Unassigned: {}", unknown_count);

        // Slot yang memang kosong di spesifikasi Dalvik standar:
        // 0x3e-0x43, 0x79, 0x7a, dll.
        // Jika unknown_count > 10, kemungkinan ada yang terlewat.
        assert!(unknown_count < 10, "Terlalu banyak opcode yang tidak teridentifikasi!");
    }
}
