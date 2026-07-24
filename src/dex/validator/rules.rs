use crate::dex::core::models::header::RawHeader;
use crate::dex::core::models::map_list::types as map_types;
use crate::dex::error::DexError;
use crate::dex::core::utils::calculate_adler32;
use crate::dex::parsers::map_list;
use crate::dex::core::constants::dex::{MAGIC_PREFIX, SUPPORTED_VERSIONS};
use super::ValidationRule;

pub struct MagicRule;
impl ValidationRule for MagicRule {
    fn validate(&self, _buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        let magic = &header.magic;
        if &magic[0..4] != MAGIC_PREFIX {
            return Err(DexError::InvalidMagic);
        }
        let version = std::str::from_utf8(&magic[4..7]).map_err(|_| DexError::InvalidVersion("Invalid UTF-8".into()))?;
        if !SUPPORTED_VERSIONS.contains(&version) {
            return Err(DexError::InvalidVersion(version.to_string()));
        }
        Ok(())
    }
}

pub struct ChecksumRule;
impl ValidationRule for ChecksumRule {
    fn validate(&self, buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        if buffer.len() < 12 { return Err(DexError::UnexpectedEOF); }
        let calculated = calculate_adler32(&buffer[12..]);
        if calculated != header.checksum {
            return Err(DexError::InvalidChecksum { expected: header.checksum, calculated });
        }
        Ok(())
    }
}

pub struct OffsetBoundsRule;
impl ValidationRule for OffsetBoundsRule {
    fn validate(&self, buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        let file_size = buffer.len() as u32;
        if header.file_size != file_size {
            return Err(DexError::InvalidOffset(format!("Header size mismatch: {} vs {}", header.file_size, file_size)));
        }
        let checks = [
            ("string_ids", header.string_ids_off, header.string_ids_size),
            ("class_defs", header.class_defs_off, header.class_defs_size),
        ];
        for (name, off, size) in checks {
            if size > 0 && off >= file_size {
                return Err(DexError::InvalidOffset(format!("{} offset out of bounds", name)));
            }
        }
        Ok(())
    }
}

pub struct MapListRule;
impl ValidationRule for MapListRule {
    fn validate(&self, buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        let map = map_list::parse(buffer, header.map_off as usize, scroll::Endian::Little)?;
        for item in &map.items {
            if item.item_type == map_types::TYPE_STRING_ID_ITEM && item.size != header.string_ids_size {
                return Err(DexError::InvalidOffset("Map item String ID mismatch".into()));
            }
        }
        Ok(())
    }
}
