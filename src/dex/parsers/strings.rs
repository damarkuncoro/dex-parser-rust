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
    /// Handles MUTF-8 by falling back to lossy decoding for stability.
    pub fn resolve_strings<'a>(buffer: &'a [u8], offsets: &[u32]) -> Result<Vec<&'a str>, DexError> {
        let mut strings = Vec::with_capacity(offsets.len());
        for &off in offsets {
            let mut curr = off as usize;
            // Read ULEB128 length (number of UTF-16 code units, not bytes)
            let (_utf16_len, bytes_read) = crate::dex::readers::leb128::read_uleb128(buffer, curr)?;
            curr += bytes_read;

            let start = curr;
            let mut end = start;
            // Find null terminator
            while end < buffer.len() && buffer[end] != 0 {
                end += 1;
            }

            let slice = &buffer[start..end];
            match std::str::from_utf8(slice) {
                Ok(s) => strings.push(s),
                Err(_) => {
                    // Fallback for MUTF-8 or obfuscated strings:
                    // Since we need to return &'a str (Zero-Copy), we can't easily fix the bytes.
                    // However, we can try to return as much valid UTF-8 as possible
                    // or use a placeholder to prevent the entire parser from failing.
                    let valid_len = std::str::from_utf8(slice)
                        .err()
                        .map(|e| e.valid_up_to())
                        .unwrap_or(0);

                    if valid_len > 0 {
                        strings.push(unsafe { std::str::from_utf8_unchecked(&slice[..valid_len]) });
                    } else {
                        // If completely invalid, use a static placeholder
                        strings.push("<invalid utf8>");
                    }
                }
            }
        }
        Ok(strings)
    }
}
