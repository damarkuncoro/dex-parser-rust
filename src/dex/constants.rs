/// Nilai konstanta sesuai spesifikasi DEX
pub const NO_INDEX: u32 = 0xffffffff;

/// Ukuran struktur data dalam byte (Fixed Sizes)
pub mod sizes {
    pub const STRING_ID_ITEM: usize = 4;
    pub const TYPE_ID_ITEM: usize = 4;
    pub const PROTO_ID_ITEM: usize = 12;
    pub const FIELD_ID_ITEM: usize = 8;
    pub const METHOD_ID_ITEM: usize = 8;
    pub const CLASS_DEF_ITEM: usize = 32;
    pub const CODE_ITEM_HEADER: usize = 16;
}

/// Offset spesifik dalam Header
pub mod offsets {
    pub const MAGIC: usize = 0;
    pub const CHECKSUM: usize = 8;
    pub const SIGNATURE: usize = 12;
    pub const FILE_SIZE: usize = 32;
    pub const HEADER_SIZE: usize = 36;
    pub const ENDIAN_TAG: usize = 40;
}

/// Tag Endianness
pub const ENDIAN_CONSTANT: u32 = 0x12345678;
pub const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;

/// DEX Metadata
pub mod dex {
    pub const MAGIC_PREFIX: &[u8; 4] = b"dex\n";
    pub const SUPPORTED_VERSIONS: &[&str] = &["035", "037", "038", "039"];
}

/// Encoded Value Types
pub mod value_types {
    pub const BYTE: u8 = 0x00;
    pub const SHORT: u8 = 0x02;
    pub const CHAR: u8 = 0x03;
    pub const INT: u8 = 0x04;
    pub const LONG: u8 = 0x06;
    pub const FLOAT: u8 = 0x10;
    pub const DOUBLE: u8 = 0x11;
    pub const METHOD_TYPE: u8 = 0x15;
    pub const METHOD_HANDLE: u8 = 0x16;
    pub const STRING: u8 = 0x17;
    pub const TYPE: u8 = 0x18;
    pub const FIELD: u8 = 0x19;
    pub const METHOD: u8 = 0x1a;
    pub const ENUM: u8 = 0x1b;
    pub const ARRAY: u8 = 0x1c;
    pub const ANNOTATION: u8 = 0x1d;
    pub const NULL: u8 = 0x1e;
    pub const BOOLEAN: u8 = 0x1f;
}

/// Debug Info Opcodes
pub mod debug {
    pub const END_SEQUENCE: u8 = 0x00;
    pub const ADVANCE_PC: u8 = 0x01;
    pub const ADVANCE_LINE: u8 = 0x02;
    pub const START_LOCAL: u8 = 0x03;
    pub const START_LOCAL_EXTENDED: u8 = 0x04;
    pub const END_LOCAL: u8 = 0x05;
    pub const RESTART_LOCAL: u8 = 0x06;
    pub const SET_PROLOGUE_END: u8 = 0x07;
    pub const SET_EPILOGUE_BEGIN: u8 = 0x08;
    pub const SET_FILE: u8 = 0x09;
    pub const FIRST_SPECIAL: u8 = 0x0a;
}

/// Access Flags Masks
pub mod access_flags {
    pub const PUBLIC: u32 = 0x0001;
    pub const PRIVATE: u32 = 0x0002;
    pub const PROTECTED: u32 = 0x0004;
    pub const STATIC: u32 = 0x0008;
    pub const FINAL: u32 = 0x0010;
    pub const SYNCHRONIZED: u32 = 0x0020;
    pub const VOLATILE: u32 = 0x0040;
    pub const BRIDGE: u32 = 0x0040;
    pub const TRANSIENT: u32 = 0x0080;
    pub const VARARGS: u32 = 0x0080;
    pub const NATIVE: u32 = 0x0100;
    pub const INTERFACE: u32 = 0x0200;
    pub const ABSTRACT: u32 = 0x0400;
    pub const STRICTFP: u32 = 0x0800;
    pub const SYNTHETIC: u32 = 0x1000;
    pub const ANNOTATION: u32 = 0x2000;
    pub const ENUM: u32 = 0x4000;
    pub const CONSTRUCTOR: u32 = 0x10000;
    pub const DECLARED_SYNCHRONIZED: u32 = 0x20000;
}
