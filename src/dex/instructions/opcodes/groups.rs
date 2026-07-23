use super::{IndexType, OpcodeInfo};

pub fn get_moves(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
    match opcode {
        0x01 => Some(("move", "vA, vB", 1, IndexType::None)),
        0x02 => Some(("move/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x03 => Some(("move/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x04 => Some(("move-wide", "vA, vB", 1, IndexType::None)),
        0x05 => Some(("move-wide/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x06 => Some(("move-wide/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x07 => Some(("move-object", "vA, vB", 1, IndexType::None)),
        0x08 => Some(("move-object/from16", "vAA, vBBBB", 2, IndexType::None)),
        0x09 => Some(("move-object/16", "vAAAA, vBBBB", 3, IndexType::None)),
        0x0a => Some(("move-result", "vAA", 1, IndexType::None)),
        0x0b => Some(("move-result-wide", "vAA", 1, IndexType::None)),
        0x0c => Some(("move-result-object", "vAA", 1, IndexType::None)),
        0x0d => Some(("move-exception", "vAA", 1, IndexType::None)),
        _ => None,
    }
}

pub fn get_consts(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
    match opcode {
        0x12 => Some(("const/4", "vA, #+B", 1, IndexType::None)),
        0x13 => Some(("const/16", "vAA, #+BBBB", 2, IndexType::None)),
        0x14 => Some(("const", "vAA, #+BBBBBBBB", 3, IndexType::None)),
        0x15 => Some(("const/high16", "vAA, #+BBBB0000", 2, IndexType::None)),
        0x16 => Some(("const-wide/16", "vAA, #+BBBB", 2, IndexType::None)),
        0x17 => Some(("const-wide/32", "vAA, #+BBBBBBBB", 3, IndexType::None)),
        0x18 => Some(("const-wide", "vAA, #+BBBBBBBBBBBBBBBB", 5, IndexType::None)),
        0x19 => Some(("const-wide/high16", "vAA, #+BBBB000000000000", 2, IndexType::None)),
        0x1a => Some(("const-string", "vAA, string@", 2, IndexType::String)),
        0x1b => Some(("const-string/jumbo", "vAAAA, string@", 3, IndexType::String)),
        0x1c => Some(("const-class", "vAA, type@", 2, IndexType::Type)),
        _ => None,
    }
}

pub fn get_flow(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
    match opcode {
        0x0e => Some(("return-void", "", 1, IndexType::None)),
        0x0f => Some(("return", "vAA", 1, IndexType::None)),
        0x10 => Some(("return-wide", "vAA", 1, IndexType::None)),
        0x11 => Some(("return-object", "vAA", 1, IndexType::None)),
        0x27 => Some(("throw", "vAA", 1, IndexType::None)),
        0x28 => Some(("goto", "+AA", 1, IndexType::None)),
        0x29 => Some(("goto/16", "+AAAA", 2, IndexType::None)),
        0x2a => Some(("goto/32", "+AAAAAAAA", 3, IndexType::None)),
        0x2b => Some(("packed-switch", "vAA, +BBBBBBBB", 3, IndexType::None)),
        0x2c => Some(("sparse-switch", "vAA, +BBBBBBBB", 3, IndexType::None)),
        _ => None,
    }
}

pub fn get_ifs(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
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

pub fn get_objects(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
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
        _ => None,
    }
}

pub fn get_fields(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
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
        _ => None,
    }
}

pub fn get_invokes(opcode: u8) -> Option<(&'static str, &'static str, usize, IndexType)> {
    match opcode {
        0x6e => Some(("invoke-virtual", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x6f => Some(("invoke-super", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x70 => Some(("invoke-direct", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x71 => Some(("invoke-static", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x72 => Some(("invoke-interface", "{vC..vG}, meth@", 3, IndexType::Method)),
        0x74 => Some(("invoke-virtual/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x75 => Some(("invoke-super/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x76 => Some(("invoke-direct/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x77 => Some(("invoke-static/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0x78 => Some(("invoke-interface/range", "{vCCCC..vNNNN}, meth@", 3, IndexType::Method)),
        0xfa => Some(("invoke-polymorphic", "{vC..vG}, meth@, proto@", 4, IndexType::Method)),
        0xfb => Some(("invoke-polymorphic/range", "{vCCCC..vNNNN}, meth@, proto@", 4, IndexType::Method)),
        0xfc => Some(("invoke-custom", "{vC..vG}, call_site@", 3, IndexType::None)),
        0xfd => Some(("invoke-custom/range", "{vCCCC..vNNNN}, call_site@", 3, IndexType::None)),
        _ => None,
    }
}
