use scroll::{Pread, LE};
use crate::dex::error::DexError;

/// A simple, fast Android Binary XML (AXML) parser foundation.
pub struct AxmlParser<'a> {
    buffer: &'a [u8],
}

#[derive(Debug)]
pub enum ChunkType {
    StringPool = 0x0001,
    ResourceMap = 0x0008,
    StartNamespace = 0x0100,
    EndNamespace = 0x0101,
    StartElement = 0x0102,
    EndElement = 0x0103,
    Text = 0x0104,
}

impl<'a> AxmlParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn parse_manifest(&self) -> Result<crate::dex::core::models::Manifest, DexError> {
        let mut manifest = crate::dex::core::models::Manifest::default();
        let mut offset = 0;

        // Magic Number: 0x00080003
        let magic: u32 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if magic != 0x00080003 {
            return Err(DexError::InvalidMagic);
        }
        offset += 4;

        // File Size
        let _file_size: u32 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        offset += 4;

        let mut string_pool = Vec::new();

        while offset < self.buffer.len() {
            let chunk_type: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
            let header_size: u16 = self.buffer.pread_with(offset + 2, LE).map_err(DexError::ScrollError)?;
            let chunk_size: u32 = self.buffer.pread_with(offset + 4, LE).map_err(DexError::ScrollError)?;

            match chunk_type {
                0x0001 => { // String Pool
                    string_pool = self.parse_string_pool(offset, chunk_size as usize)?;
                }
                0x0102 => { // Start Element
                    self.process_start_element(offset, header_size, &string_pool, &mut manifest)?;
                }
                _ => {}
            }

            offset += chunk_size as usize;
            if chunk_size == 0 { break; } // Safety break
        }

        Ok(manifest)
    }

    fn parse_string_pool(&self, offset: usize, _size: usize) -> Result<Vec<String>, DexError> {
        let mut strings = Vec::new();
        let string_count: u32 = self.buffer.pread_with(offset + 8, LE).map_err(DexError::ScrollError)?;
        let flags: u32 = self.buffer.pread_with(offset + 16, LE).map_err(DexError::ScrollError)?;
        let strings_start: u32 = self.buffer.pread_with(offset + 20, LE).map_err(DexError::ScrollError)?;

        let is_utf8 = (flags & 0x100) != 0;

        for i in 0..string_count {
            let string_offset: u32 = self.buffer.pread_with(offset + 28 + (i as usize * 4), LE).map_err(DexError::ScrollError)?;
            let mut current_offset = offset + strings_start as usize + string_offset as usize;

            if is_utf8 {
                // AXML UTF-8 strings have TWO length bytes: [char count][byte count]
                let (_char_len, bytes_read_1) = self.decode_axml_len(current_offset)?;
                current_offset += bytes_read_1;
                let (byte_len, bytes_read_2) = self.decode_axml_len(current_offset)?;
                current_offset += bytes_read_2;

                if current_offset + byte_len > self.buffer.len() {
                    strings.push(String::new());
                    continue;
                }

                let s_bytes = &self.buffer[current_offset..current_offset + byte_len];
                strings.push(String::from_utf8_lossy(s_bytes).to_string());
            } else {
                // UTF-16
                let (char_len, bytes_read) = self.decode_axml_len_u16(current_offset)?;
                current_offset += bytes_read;
                let byte_len = char_len * 2;

                if current_offset + byte_len > self.buffer.len() {
                    strings.push(String::new());
                    continue;
                }

                let mut utf16_data = Vec::with_capacity(char_len);
                for j in 0..char_len {
                    let val: u16 = self.buffer.pread_with(current_offset + (j * 2), LE).map_err(DexError::ScrollError)?;
                    utf16_data.push(val);
                }
                strings.push(String::from_utf16_lossy(&utf16_data));
            }
        }

        Ok(strings)
    }

    /// Decodes AXML length encoding (1 or 2 bytes) - for UTF-8 bytes
    fn decode_axml_len(&self, offset: usize) -> Result<(usize, usize), DexError> {
        let first: u8 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if (first & 0x80) != 0 {
            let second: u8 = self.buffer.pread_with(offset + 1, LE).map_err(DexError::ScrollError)?;
            Ok(((((first & 0x7f) as usize) << 8) | (second as usize), 2))
        } else {
            Ok((first as usize, 1))
        }
    }

    /// Decodes AXML length encoding (1 or 2 u16s) - for UTF-16 chars
    fn decode_axml_len_u16(&self, offset: usize) -> Result<(usize, usize), DexError> {
        let first: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if (first & 0x8000) != 0 {
            let second: u16 = self.buffer.pread_with(offset + 2, LE).map_err(DexError::ScrollError)?;
            Ok(((((first & 0x7fff) as usize) << 16) | (second as usize), 4))
        } else {
            Ok((first as usize, 2))
        }
    }

    fn process_start_element(&self, offset: usize, _header_size: u16, string_pool: &[String], manifest: &mut crate::dex::core::models::Manifest) -> Result<(), DexError> {
        let name_idx: u32 = self.buffer.pread_with(offset + 16, LE).map_err(DexError::ScrollError)?;
        let name = string_pool.get(name_idx as usize).cloned().unwrap_or_default();

        let attr_count: u16 = self.buffer.pread_with(offset + 28, LE).map_err(DexError::ScrollError)?;
        let attr_start = offset + 36;

        for i in 0..attr_count {
            let attr_offset = attr_start + (i as usize * 20);
            let attr_name_idx: u32 = self.buffer.pread_with(attr_offset + 4, LE).map_err(DexError::ScrollError)?;
            let attr_val_idx: i32 = self.buffer.pread_with(attr_offset + 8, LE).map_err(DexError::ScrollError)?;

            let attr_name = string_pool.get(attr_name_idx as usize).map(|s| s.as_str()).unwrap_or("");
            let attr_value = if attr_val_idx >= 0 {
                string_pool.get(attr_val_idx as usize).cloned().unwrap_or_default()
            } else {
                String::new()
            };

            match name.as_str() {
                "manifest" => {
                    if attr_name == "package" { manifest.package_name = attr_value; }
                }
                "uses-permission" => {
                    if attr_name == "name" { manifest.permissions.push(attr_value); }
                }
                "activity" => {
                    if attr_name == "name" { manifest.activities.push(attr_value); }
                }
                "service" => {
                    if attr_name == "name" { manifest.services.push(attr_value); }
                }
                "receiver" => {
                    if attr_name == "name" { manifest.receivers.push(attr_value); }
                }
                "provider" => {
                    if attr_name == "name" { manifest.providers.push(attr_value); }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
