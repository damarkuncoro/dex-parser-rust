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
    pub const ENDIAN_TAG: usize = 40;
}

/// Tag Endianness
pub const ENDIAN_CONSTANT: u32 = 0x12345678;
pub const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;
