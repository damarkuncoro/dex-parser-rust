use crate::dex::error::DexError;
use crate::dex::readers::DexReader;

pub struct StringSection;

impl StringSection {
    pub fn parse_offsets(reader: &mut DexReader, size: u32, offset: u32) -> Result<Vec<u32>, DexError> {
        reader.seek(offset as usize)?;
        let mut offsets = Vec::with_capacity(size as usize);
        for _ in 0..size {
            offsets.push(reader.read_u32()?);
        }
        Ok(offsets)
    }

    /// Specialty: Returns raw byte slices (Zero-Copy).
    /// No UTF-8 validation or String allocation happens here.
    pub fn resolve_strings<'a>(reader: &mut DexReader<'a>, offsets: &[u32]) -> Result<Vec<&'a [u8]>, DexError> {
        let mut strings = Vec::with_capacity(offsets.len());
        for &off in offsets {
            reader.seek(off as usize)?;
            let _utf16_len = reader.read_uleb128()?;

            let start = reader.position();
            let mut end = start;
            let buffer = reader.buffer();
            while end < buffer.len() && buffer[end] != 0 {
                end += 1;
            }
            let len = end - start;
            // Read the bytes including null terminator to mark them as used
            let bytes = reader.read_bytes(len + 1)?;
            strings.push(&bytes[..len]);
        }
        Ok(strings)
    }
}
