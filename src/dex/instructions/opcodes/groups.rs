use super::{IndexType};

pub type OpcodeData = (&'static str, &'static str, usize, IndexType);

pub fn get_basics(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x00 => Some(("nop", "", 1, IndexType::None)),
        _ => None,
    }
}

pub fn get_moves(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_consts(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_flow(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_ifs(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_objects(opcode: u8) -> Option<OpcodeData> {
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

        // AGET
        0x44 => Some(("aget", "vAA, vBB, vCC", 2, IndexType::None)),
        0x45 => Some(("aget-wide", "vAA, vBB, vCC", 2, IndexType::None)),
        0x46 => Some(("aget-object", "vAA, vBB, vCC", 2, IndexType::None)),
        0x47 => Some(("aget-boolean", "vAA, vBB, vCC", 2, IndexType::None)),
        0x48 => Some(("aget-byte", "vAA, vBB, vCC", 2, IndexType::None)),
        0x49 => Some(("aget-char", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4a => Some(("aget-short", "vAA, vBB, vCC", 2, IndexType::None)),

        // APUT
        0x4b => Some(("aput", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4c => Some(("aput-wide", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4d => Some(("aput-object", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4e => Some(("aput-boolean", "vAA, vBB, vCC", 2, IndexType::None)),
        0x4f => Some(("aput-byte", "vAA, vBB, vCC", 2, IndexType::None)),
        0x50 => Some(("aput-char", "vAA, vBB, vCC", 2, IndexType::None)),
        0x51 => Some(("aput-short", "vAA, vBB, vCC", 2, IndexType::None)),
        _ => None,
    }
}

pub fn get_fields(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_invokes(opcode: u8) -> Option<OpcodeData> {
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

pub fn get_arithmetic(opcode: u8) -> Option<OpcodeData> {
    match opcode {
        0x7b..=0x8f => Some(get_unary(opcode)),
        0x90..=0xaf => Some(get_binop(opcode)),
        0xb0..=0xcf => Some(get_binop_2addr(opcode)),
        0xd0..=0xff => Some(get_binop_lit(opcode)),
        _ => None,
    }
}

fn get_unary(opcode: u8) -> OpcodeData {
    match opcode {
        0x7b => ("neg-int", "vA, vB", 1, IndexType::None),
        0x7c => ("not-int", "vA, vB", 1, IndexType::None),
        0x7d => ("neg-long", "vA, vB", 1, IndexType::None),
        0x7e => ("not-long", "vA, vB", 1, IndexType::None),
        0x7f => ("neg-float", "vA, vB", 1, IndexType::None),
        0x80 => ("neg-double", "vA, vB", 1, IndexType::None),
        0x81 => ("int-to-long", "vA, vB", 1, IndexType::None),
        0x82 => ("int-to-float", "vA, vB", 1, IndexType::None),
        0x83 => ("int-to-double", "vA, vB", 1, IndexType::None),
        0x84 => ("long-to-int", "vA, vB", 1, IndexType::None),
        0x85 => ("long-to-float", "vA, vB", 1, IndexType::None),
        0x86 => ("long-to-double", "vA, vB", 1, IndexType::None),
        0x87 => ("float-to-int", "vA, vB", 1, IndexType::None),
        0x88 => ("float-to-long", "vA, vB", 1, IndexType::None),
        0x89 => ("float-to-double", "vA, vB", 1, IndexType::None),
        0x8a => ("double-to-int", "vA, vB", 1, IndexType::None),
        0x8b => ("double-to-long", "vA, vB", 1, IndexType::None),
        0x8c => ("double-to-float", "vA, vB", 1, IndexType::None),
        0x8d => ("int-to-byte", "vA, vB", 1, IndexType::None),
        0x8e => ("int-to-char", "vA, vB", 1, IndexType::None),
        0x8f => ("int-to-short", "vA, vB", 1, IndexType::None),
        _ => ("unknown", "...", 1, IndexType::None),
    }
}

fn get_binop(opcode: u8) -> OpcodeData {
    match opcode {
        0x90 => ("add-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x91 => ("sub-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x92 => ("mul-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x93 => ("div-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x94 => ("rem-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x95 => ("and-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x96 => ("or-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x97 => ("xor-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x98 => ("shl-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x99 => ("shr-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x9a => ("ushr-int", "vAA, vBB, vCC", 2, IndexType::None),
        0x9b => ("add-long", "vAA, vBB, vCC", 2, IndexType::None),
        0x9c => ("sub-long", "vAA, vBB, vCC", 2, IndexType::None),
        0x9d => ("mul-long", "vAA, vBB, vCC", 2, IndexType::None),
        0x9e => ("div-long", "vAA, vBB, vCC", 2, IndexType::None),
        0x9f => ("rem-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa0 => ("and-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa1 => ("or-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa2 => ("xor-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa3 => ("shl-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa4 => ("shr-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa5 => ("ushr-long", "vAA, vBB, vCC", 2, IndexType::None),
        0xa6 => ("add-float", "vAA, vBB, vCC", 2, IndexType::None),
        0xa7 => ("sub-float", "vAA, vBB, vCC", 2, IndexType::None),
        0xa8 => ("mul-float", "vAA, vBB, vCC", 2, IndexType::None),
        0xa9 => ("div-float", "vAA, vBB, vCC", 2, IndexType::None),
        0xaa => ("rem-float", "vAA, vBB, vCC", 2, IndexType::None),
        0xab => ("add-double", "vAA, vBB, vCC", 2, IndexType::None),
        0xac => ("sub-double", "vAA, vBB, vCC", 2, IndexType::None),
        0xad => ("mul-double", "vAA, vBB, vCC", 2, IndexType::None),
        0xae => ("div-double", "vAA, vBB, vCC", 2, IndexType::None),
        0xaf => ("rem-double", "vAA, vBB, vCC", 2, IndexType::None),
        _ => ("unknown", "...", 1, IndexType::None),
    }
}

fn get_binop_2addr(opcode: u8) -> OpcodeData {
    match opcode {
        0xb0 => ("add-int/2addr", "vA, vB", 1, IndexType::None),
        0xb1 => ("sub-int/2addr", "vA, vB", 1, IndexType::None),
        0xb2 => ("mul-int/2addr", "vA, vB", 1, IndexType::None),
        0xb3 => ("div-int/2addr", "vA, vB", 1, IndexType::None),
        0xb4 => ("rem-int/2addr", "vA, vB", 1, IndexType::None),
        0xb5 => ("and-int/2addr", "vA, vB", 1, IndexType::None),
        0xb6 => ("or-int/2addr", "vA, vB", 1, IndexType::None),
        0xb7 => ("xor-int/2addr", "vA, vB", 1, IndexType::None),
        0xb8 => ("shl-int/2addr", "vA, vB", 1, IndexType::None),
        0xb9 => ("shr-int/2addr", "vA, vB", 1, IndexType::None),
        0xba => ("ushr-int/2addr", "vA, vB", 1, IndexType::None),
        0xbb => ("add-long/2addr", "vA, vB", 1, IndexType::None),
        0xbc => ("sub-long/2addr", "vA, vB", 1, IndexType::None),
        0xbd => ("mul-long/2addr", "vA, vB", 1, IndexType::None),
        0xbe => ("div-long/2addr", "vA, vB", 1, IndexType::None),
        0xbf => ("rem-long/2addr", "vA, vB", 1, IndexType::None),
        0xc0 => ("and-long/2addr", "vA, vB", 1, IndexType::None),
        0xc1 => ("or-long/2addr", "vA, vB", 1, IndexType::None),
        0xc2 => ("xor-long/2addr", "vA, vB", 1, IndexType::None),
        0xc3 => ("shl-long/2addr", "vA, vB", 1, IndexType::None),
        0xc4 => ("shr-long/2addr", "vA, vB", 1, IndexType::None),
        0xc5 => ("ushr-long/2addr", "vA, vB", 1, IndexType::None),
        0xc6 => ("add-float/2addr", "vA, vB", 1, IndexType::None),
        0xc7 => ("sub-float/2addr", "vA, vB", 1, IndexType::None),
        0xc8 => ("mul-float/2addr", "vA, vB", 1, IndexType::None),
        0xc9 => ("div-float/2addr", "vA, vB", 1, IndexType::None),
        0xca => ("rem-float/2addr", "vA, vB", 1, IndexType::None),
        0xcb => ("add-double/2addr", "vA, vB", 1, IndexType::None),
        0xcc => ("sub-double/2addr", "vA, vB", 1, IndexType::None),
        0xcd => ("mul-double/2addr", "vA, vB", 1, IndexType::None),
        0xce => ("div-double/2addr", "vA, vB", 1, IndexType::None),
        0xcf => ("rem-double/2addr", "vA, vB", 1, IndexType::None),
        _ => ("unknown", "...", 1, IndexType::None),
    }
}

fn get_binop_lit(opcode: u8) -> OpcodeData {
    match opcode {
        0xd0 => ("add-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd1 => ("rsub-int", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd2 => ("mul-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd3 => ("div-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd4 => ("rem-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd5 => ("and-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd6 => ("or-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd7 => ("xor-int/lit16", "vA, vB, #+CCCC", 2, IndexType::None),
        0xd8 => ("add-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xd9 => ("rsub-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xda => ("mul-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xdb => ("div-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xdc => ("rem-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xdd => ("and-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xde => ("or-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xdf => ("xor-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xe0 => ("shl-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xe1 => ("shr-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        0xe2 => ("ushr-int/lit8", "vAA, vBB, #+CC", 2, IndexType::None),
        _ => ("unknown", "...", 1, IndexType::None),
    }
}
