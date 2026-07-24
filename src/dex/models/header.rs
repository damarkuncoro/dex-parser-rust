use scroll::Pread;
use serde::{Serialize};

#[derive(Debug, Pread, Serialize,  Clone)]
pub struct RawHeader {
    pub magic: [u8; 8],
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    pub endian_tag: u32,
    pub link_size: u32,
    pub link_off: u32,
    pub map_off: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub proto_ids_size: u32,
    pub proto_ids_off: u32,
    pub field_ids_size: u32,
    pub field_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
    pub data_size: u32,
    pub data_off: u32,
}

#[derive(Debug, Serialize,  Clone, Default)]
pub struct ModernHeaderExt {
    pub method_handles_size: u32,
    pub method_handles_off: u32,
    pub call_site_ids_size: u32,
    pub call_site_ids_off: u32,
}
