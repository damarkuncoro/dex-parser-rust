pub mod sizes;
pub mod offsets;
pub mod dex;
pub mod value_types;
pub mod debug;
pub mod access_flags;

/// Nilai konstanta sesuai spesifikasi DEX
pub const NO_INDEX: u32 = 0xffffffff;

/// Tag Endianness
pub const ENDIAN_CONSTANT: u32 = 0x12345678;
pub const REVERSE_ENDIAN_CONSTANT: u32 = 0x78563412;
