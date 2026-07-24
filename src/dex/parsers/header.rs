use crate::dex::error::DexError;
use crate::dex::core::models::header::{RawHeader, RawCdexHeader};
use crate::dex::readers::DexReader;
use crate::dex::core::constants::dex as dex_consts;
use scroll::Pread;

pub enum HeaderType {
    Standard(RawHeader),
    Compact(RawCdexHeader),
}

impl HeaderType {
    /// Safely extract the common header fields shared between Standard and Compact DEX.
    pub fn common(&self) -> RawHeader {
        match self {
            HeaderType::Standard(h) => h.clone(),
            HeaderType::Compact(h) => RawHeader {
                magic: h.magic,
                checksum: h.checksum,
                signature: h.signature,
                file_size: h.file_size,
                header_size: h.header_size,
                endian_tag: h.endian_tag,
                link_size: h.link_size,
                link_off: h.link_off,
                map_off: h.map_off,
                string_ids_size: h.string_ids_size,
                string_ids_off: h.string_ids_off,
                type_ids_size: h.type_ids_size,
                type_ids_off: h.type_ids_off,
                proto_ids_size: h.proto_ids_size,
                proto_ids_off: h.proto_ids_off,
                field_ids_size: h.field_ids_size,
                field_ids_off: h.field_ids_off,
                method_ids_size: h.method_ids_size,
                method_ids_off: h.method_ids_off,
                class_defs_size: h.class_defs_size,
                class_defs_off: h.class_defs_off,
                data_size: h.data_size,
                data_off: h.data_off,
            },
        }
    }
}

pub struct HeaderParser;

impl HeaderParser {
    pub fn parse(reader: &mut DexReader) -> Result<HeaderType, DexError> {
        reader.seek(0)?;
        let magic: [u8; 8] = reader.read_bytes(8)?.try_into().unwrap();

        reader.seek(0)?;
        if magic.starts_with(dex_consts::CDEX_MAGIC_PREFIX) {
            let bytes = reader.read_bytes(128)?;
            let header: RawCdexHeader = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            Ok(HeaderType::Compact(header))
        } else {
            let bytes = reader.read_bytes(112)?;
            let header: RawHeader = bytes.pread_with(0, reader.endian()).map_err(DexError::ScrollError)?;
            Ok(HeaderType::Standard(header))
        }
    }
}
