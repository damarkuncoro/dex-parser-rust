use scroll::{Pread, Endian};
use crate::dex::error::DexError;
use crate::dex::utils::read_uleb128;
use crate::dex::constants::sizes::STRING_ID_ITEM;

pub struct StringSection;

impl StringSection {
    pub fn parse(buffer: &[u8], header_string_ids_size: u32, header_string_ids_off: u32, endian: Endian) -> Result<Vec<String>, DexError> {
        let size = header_string_ids_size as usize;
        let offset = header_string_ids_off as usize;

        let mut strings = Vec::with_capacity(size);
        for i in 0..size {
            let off = offset + (i * STRING_ID_ITEM);
            let data_off: u32 = buffer.pread_with(off, endian)?;

            let mut curr = data_off as usize;
            let (_len, bytes_read) = read_uleb128(buffer, curr);
            curr += bytes_read;

            let mut end = curr;
            while end < buffer.len() && buffer[end] != 0 {
                end += 1;
            }
            strings.push(String::from_utf8_lossy(&buffer[curr..end]).to_string());
        }
        Ok(strings)
    }
}

pub fn parse(buffer: &[u8], header_string_ids_size: u32, header_string_ids_off: u32, endian: Endian) -> Result<Vec<String>, DexError> {
    StringSection::parse(buffer, header_string_ids_size, header_string_ids_off, endian)
}
