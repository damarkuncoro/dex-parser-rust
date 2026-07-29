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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InstructionFormat {
    Unknown = 0,
    Fmt10x, Fmt12x, Fmt11n, Fmt11x, Fmt10t,
    Fmt20bc, Fmt20t, Fmt22x, Fmt21t, Fmt21s, Fmt21h, Fmt21c, Fmt23x, Fmt22b, Fmt22t, Fmt22s, Fmt22c, Fmt22cs,
    Fmt32x, Fmt30t, Fmt31t, Fmt31i, Fmt31c, Fmt35c, Fmt35ms, Fmt35fs, Fmt3rc, Fmt3rms, Fmt3rfs, Fmt3inline,
    Fmt51l,
}

#[derive(Clone, Debug)]
pub struct OpcodeInfo {
    pub name: String,
    pub format: InstructionFormat,
    pub length: usize,
    pub index_type: IndexType,
    // Official AOSP Flags from InstrUtils.h
    pub can_branch: bool,
    pub can_continue: bool,
    pub can_switch: bool,
    pub can_throw: bool,
    pub can_return: bool,

    pub is_unused_spec: bool,
}

static OPCODE_TABLE: Lazy<Vec<OpcodeInfo>> = Lazy::new(|| {
    let mut table = Vec::with_capacity(256);
    for opcode in 0..=255 {
        let op = opcode as u8;
        let data = groups::get_opcode_data(op);

        let is_unused = is_unused_in_dalvik_spec(op);

        let info = match data {
            Some(d) => OpcodeInfo {
                name: d.0.to_string(),
                format: parse_format_str(d.1),
                length: d.2,
                index_type: d.3,
                can_branch: is_branch_op(op),
                can_continue: is_continue_op(op),
                can_switch: is_switch_op(op),
                can_throw: true,
                can_return: is_return_op(op),
                is_unused_spec: is_unused,
            },
            None => OpcodeInfo {
                name: if is_unused { format!("UNUSED_{:02X}", op) } else { format!("op_{:02x}", op) },
                format: InstructionFormat::Unknown,
                length: 1,
                index_type: IndexType::None,
                can_branch: false,
                can_continue: true,
                can_switch: false,
                can_throw: false,
                can_return: false,
                is_unused_spec: is_unused,
            }
        };
        table.push(info);
    }
    table
});

fn parse_format_str(s: &str) -> InstructionFormat {
    match s {
        "10x" => InstructionFormat::Fmt10x,
        "12x" => InstructionFormat::Fmt12x,
        "11n" => InstructionFormat::Fmt11n,
        "11x" => InstructionFormat::Fmt11x,
        "10t" => InstructionFormat::Fmt10t,
        "20bc" => InstructionFormat::Fmt20bc,
        "20t" => InstructionFormat::Fmt20t,
        "22x" => InstructionFormat::Fmt22x,
        "21t" => InstructionFormat::Fmt21t,
        "21s" => InstructionFormat::Fmt21s,
        "21h" => InstructionFormat::Fmt21h,
        "21c" => InstructionFormat::Fmt21c,
        "23x" => InstructionFormat::Fmt23x,
        "22b" => InstructionFormat::Fmt22b,
        "22t" => InstructionFormat::Fmt22t,
        "22s" => InstructionFormat::Fmt22s,
        "22c" => InstructionFormat::Fmt22c,
        "22cs" => InstructionFormat::Fmt22cs,
        "32x" => InstructionFormat::Fmt32x,
        "30t" => InstructionFormat::Fmt30t,
        "31t" => InstructionFormat::Fmt31t,
        "31i" => InstructionFormat::Fmt31i,
        "31c" => InstructionFormat::Fmt31c,
        "35c" => InstructionFormat::Fmt35c,
        "35ms" => InstructionFormat::Fmt35ms,
        "35fs" => InstructionFormat::Fmt35fs,
        "3rc" => InstructionFormat::Fmt3rc,
        "3rms" => InstructionFormat::Fmt3rms,
        "3rfs" => InstructionFormat::Fmt3rfs,
        "3inline" => InstructionFormat::Fmt3inline,
        "51l" => InstructionFormat::Fmt51l,
        _ => InstructionFormat::Unknown,
    }
}

fn is_continue_op(op: u8) -> bool {
    match op {
        0x0e..=0x11 => false, // return*
        0x27 => false,        // throw
        0x28..=0x2a => false, // goto (absolute)
        _ => true,
    }
}

fn is_branch_op(op: u8) -> bool {
    match op {
        0x32..=0x3d | 0x28..=0x2a => true, // if-* and goto*
        _ => false,
    }
}

fn is_return_op(op: u8) -> bool {
    match op {
        0x0e..=0x11 => true,
        _ => false,
    }
}

fn is_switch_op(op: u8) -> bool {
    match op {
        0x2b..=0x2c => true, // packed-switch, sparse-switch
        _ => false,
    }
}

fn is_unused_in_dalvik_spec(op: u8) -> bool {
    match op {
        0x3e..=0x43 | 0x73 | 0x79 | 0x7a | 0xe3..=0xec | 0xef | 0xf1 | 0xfc..=0xff => true,
        _ => false,
    }
}

pub struct OpcodeTable;

impl OpcodeTable {
    pub fn get(opcode: u8) -> OpcodeInfo {
        OPCODE_TABLE[opcode as usize].clone()
    }
}
