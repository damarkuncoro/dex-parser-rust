use scroll::{Pread, LE};
use crate::dex::error::DexError;
use std::collections::HashMap;

/// A foundation for parsing Android resources.arsc files.
#[derive(Default, serde::Serialize, Clone)]
pub struct ResourceTable {
    pub package_names: Vec<String>,
    /// Maps Resource ID (e.g. 0x7f010001) to a human-readable name (e.g. "string/app_name")
    pub id_map: HashMap<u32, String>,
}

pub struct ArscParser<'a> {
    buffer: &'a [u8],
}

impl<'a> ArscParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn parse(&self) -> Result<ResourceTable, DexError> {
        let mut offset = 0;
        let mut table = ResourceTable::default();

        // Header: Type (2), Header Size (2), Chunk Size (4)
        let chunk_type: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if chunk_type != 0x0002 { // RES_TABLE_TYPE
            return Err(DexError::InvalidMagic);
        }

        offset += 12; // Skip Table Header

        while offset < self.buffer.len() {
            let inner_type: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
            let inner_size: u32 = self.buffer.pread_with(offset + 4, LE).map_err(DexError::ScrollError)?;

            match inner_type {
                0x0200 => { // Package Chunk
                    self.parse_package_chunk(offset, &mut table)?;
                }
                _ => {}
            }

            offset += inner_size as usize;
            if inner_size == 0 { break; }
        }

        Ok(table)
    }

    fn parse_package_chunk(&self, offset: usize, table: &mut ResourceTable) -> Result<(), DexError> {
        let package_id: u32 = self.buffer.pread_with(offset + 8, LE).map_err(DexError::ScrollError)?;

        let mut name_bytes = [0u16; 128];
        for i in 0..128 {
            name_bytes[i] = self.buffer.pread_with(offset + 12 + (i * 2), LE).map_err(DexError::ScrollError)?;
        }
        let len = name_bytes.iter().position(|&c| c == 0).unwrap_or(128);
        let package_name = String::from_utf16_lossy(&name_bytes[..len]);
        table.package_names.push(package_name);

        let type_strings_offset: u32 = self.buffer.pread_with(offset + 268, LE).map_err(DexError::ScrollError)?;
        let key_strings_offset: u32 = self.buffer.pread_with(offset + 276, LE).map_err(DexError::ScrollError)?;

        let type_pool = self.parse_string_pool(offset + type_strings_offset as usize)?;
        let key_pool = self.parse_string_pool(offset + key_strings_offset as usize)?;

        // Now we need to find Type chunks to build the ID map
        let mut inner_offset = offset + self.buffer.pread_with::<u16>(offset + 2, LE).unwrap() as usize;
        let package_end = offset + self.buffer.pread_with::<u32>(offset + 4, LE).unwrap() as usize;

        while inner_offset < package_end && inner_offset < self.buffer.len() {
            let chunk_type: u16 = self.buffer.pread_with(inner_offset, LE).map_err(DexError::ScrollError)?;
            let chunk_size: u32 = self.buffer.pread_with(inner_offset + 4, LE).map_err(DexError::ScrollError)?;

            if chunk_type == 0x0201 { // RES_TABLE_TYPE_TYPE
                let type_id: u8 = self.buffer.pread_with(inner_offset + 8, LE).map_err(DexError::ScrollError)?;
                let entry_count: u32 = self.buffer.pread_with(inner_offset + 12, LE).map_err(DexError::ScrollError)?;
                let entries_start: u32 = self.buffer.pread_with(inner_offset + 16, LE).map_err(DexError::ScrollError)?;

                let type_name = type_pool.get(type_id as usize - 1).map(|s| s.as_str()).unwrap_or("unknown");

                for i in 0..entry_count {
                    let entry_offset: u32 = self.buffer.pread_with(inner_offset + 20 + (i as usize * 4), LE).map_err(DexError::ScrollError)?;
                    if entry_offset == 0xffffffff { continue; }

                    let absolute_entry_offset = inner_offset + entries_start as usize + entry_offset as usize;
                    let key_idx: u32 = self.buffer.pread_with(absolute_entry_offset + 4, LE).map_err(DexError::ScrollError)?;

                    if let Some(key_name) = key_pool.get(key_idx as usize) {
                        let res_id = (package_id << 24) | ((type_id as u32) << 16) | (i as u32);
                        table.id_map.insert(res_id, format!("{}/{}", type_name, key_name));
                    }
                }
            }

            inner_offset += chunk_size as usize;
            if chunk_size == 0 { break; }
        }

        Ok(())
    }

    fn parse_string_pool(&self, offset: usize) -> Result<Vec<String>, DexError> {
        let mut strings = Vec::new();
        let string_count: u32 = self.buffer.pread_with(offset + 8, LE).map_err(DexError::ScrollError)?;
        let strings_start: u32 = self.buffer.pread_with(offset + 20, LE).map_err(DexError::ScrollError)?;
        let flags: u32 = self.buffer.pread_with(offset + 16, LE).map_err(DexError::ScrollError)?;
        let is_utf8 = (flags & 0x100) != 0;

        for i in 0..string_count {
            let string_offset: u32 = self.buffer.pread_with(offset + 28 + (i as usize * 4), LE).map_err(DexError::ScrollError)?;
            let current_offset = offset + strings_start as usize + string_offset as usize;

            if is_utf8 {
                let mut end = current_offset;
                while end < self.buffer.len() && self.buffer[end] != 0 {
                    end += 1;
                }
                let start = (current_offset + 2).min(end);
                strings.push(String::from_utf8_lossy(&self.buffer[start..end]).to_string());
            } else {
                let char_len: u16 = self.buffer.pread_with(current_offset, LE).map_err(DexError::ScrollError)?;
                let mut utf16_data = Vec::with_capacity(char_len as usize);
                for j in 0..char_len {
                    let val: u16 = self.buffer.pread_with(current_offset + 2 + (j as usize * 2), LE).map_err(DexError::ScrollError)?;
                    utf16_data.push(val);
                }
                strings.push(String::from_utf16_lossy(&utf16_data));
            }
        }
        Ok(strings)
    }
}
