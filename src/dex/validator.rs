use crate::dex::models::header::RawHeader;
use crate::dex::error::DexError;
use crate::dex::utils::calculate_adler32;
use crate::dex::constants::{ENDIAN_CONSTANT, REVERSE_ENDIAN_CONSTANT};

/// Validator for DEX file structure and integrity.
pub struct DexValidator;

impl DexValidator {
    /// Validates the entire DEX structure based on header and raw buffer.
    pub fn validate(buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        Self::validate_magic(header)?;
        Self::validate_checksum(buffer, header)?;
        Self::validate_endian_tag(header)?;
        Self::validate_offsets(buffer, header)?;
        Self::validate_alignments(header)?;

        Ok(())
    }

    /// Validates DEX Magic Number and Version
    fn validate_magic(header: &RawHeader) -> Result<(), DexError> {
        let magic = &header.magic;
        if &magic[0..3] != b"dex" || magic[3] != b'\n' {
            return Err(DexError::InvalidIndex("Invalid DEX magic number".to_string()));
        }

        let version = std::str::from_utf8(&magic[4..7])
            .map_err(|_| DexError::InvalidIndex("Invalid DEX version encoding".to_string()))?;

        match version {
            "035" | "037" | "038" | "039" => Ok(()),
            _ => Err(DexError::InvalidIndex(format!("Unsupported DEX version: {}", version))),
        }
    }

    /// Validates Adler32 Checksum
    fn validate_checksum(buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        if buffer.len() < 12 {
            return Err(DexError::InvalidIndex("File too small for checksum validation".to_string()));
        }

        let calculated = calculate_adler32(&buffer[12..]);
        if calculated != header.checksum {
            return Err(DexError::InvalidIndex(format!(
                "Checksum mismatch: expected {:08x}, calculated {:08x}",
                header.checksum, calculated
            )));
        }
        Ok(())
    }

    /// Validates Endian Tag
    fn validate_endian_tag(header: &RawHeader) -> Result<(), DexError> {
        if header.endian_tag != ENDIAN_CONSTANT && header.endian_tag != REVERSE_ENDIAN_CONSTANT {
            return Err(DexError::InvalidIndex(format!(
                "Invalid endian_tag: {:08x}", header.endian_tag
            )));
        }
        Ok(())
    }

    /// Validates Basic Offsets and Table Sizes
    fn validate_offsets(buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        let file_size = buffer.len() as u32;

        if header.file_size != file_size {
            return Err(DexError::InvalidIndex(format!(
                "Header file_size ({}) does not match actual buffer size ({})",
                header.file_size, file_size
            )));
        }

        let checks = [
            ("string_ids", header.string_ids_off, header.string_ids_size),
            ("type_ids", header.type_ids_off, header.type_ids_size),
            ("proto_ids", header.proto_ids_off, header.proto_ids_size),
            ("field_ids", header.field_ids_off, header.field_ids_size),
            ("method_ids", header.method_ids_off, header.method_ids_size),
            ("class_defs", header.class_defs_off, header.class_defs_size),
        ];

        for (name, off, size) in checks {
            if size > 0 && off >= file_size {
                return Err(DexError::InvalidIndex(format!(
                    "Offset for {} ({:08x}) is out of file bounds", name, off
                )));
            }
        }

        Ok(())
    }

    /// Validates 4-byte Alignments for specific sections
    fn validate_alignments(header: &RawHeader) -> Result<(), DexError> {
        let alignments = [
            ("string_ids", header.string_ids_off),
            ("type_ids", header.type_ids_off),
            ("proto_ids", header.proto_ids_off),
            ("field_ids", header.field_ids_off),
            ("method_ids", header.method_ids_off),
            ("class_defs", header.class_defs_off),
            ("map_off", header.map_off),
        ];

        for (name, off) in alignments {
            if off != 0 && (off % 4) != 0 {
                return Err(DexError::InvalidIndex(format!(
                    "Section {} at {:08x} is not 4-byte aligned", name, off
                )));
            }
        }
        Ok(())
    }
}
