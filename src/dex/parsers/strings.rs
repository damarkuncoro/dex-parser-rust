use crate::dex::error::DexError;
use crate::dex::readers::DexReader;

pub struct StringSection;

impl StringSection {
    /// Step 1: Parse only the string data offsets.
    pub fn parse_offsets(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<u32>, DexError> {
        reader.seek(offset as usize)?;
        let mut offsets = Vec::with_capacity(size as usize);
        for _ in 0..size {
            offsets.push(reader.read_u32()?);
        }
        Ok(offsets)
    }

    /// Step 2: Resolve actual string values from offsets (Zero-Copy).
    pub fn resolve_strings<'a>(buffer: &'a [u8], offsets: &[u32]) -> Result<Vec<&'a str>, DexError> {
        let mut strings = Vec::with_capacity(offsets.len());
        for &off in offsets {
            let mut curr = off as usize;
            // Read ULEB128 length
            let (_utf16_len, bytes_read) = crate::dex::readers::leb128::read_uleb128(buffer, curr)?;
            curr += bytes_read;

            let start = curr;
            let mut end = start;
            while end < buffer.len() && buffer[end] != 0 {
                end += 1;
            }

            let s = std::str::from_utf8(&buffer[start..end])
                .map_err(DexError::Utf8Error)?;
            strings.push(s);
        }
        Ok(strings)
    }
}
