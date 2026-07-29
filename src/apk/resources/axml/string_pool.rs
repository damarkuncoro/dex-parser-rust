use scroll::{Pread, LE};
use crate::dex::error::DexError;

pub struct StringPoolDecoder<'a> {
    buffer: &'a [u8],
}

impl<'a> StringPoolDecoder<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn decode(&self, offset: usize) -> Result<Vec<String>, DexError> {
        let mut strings = Vec::new();
        let string_count: u32 = self.buffer.pread_with(offset + 8, LE).map_err(DexError::ScrollError)?;
        let flags: u32 = self.buffer.pread_with(offset + 16, LE).map_err(DexError::ScrollError)?;
        let strings_start: u32 = self.buffer.pread_with(offset + 20, LE).map_err(DexError::ScrollError)?;

        let is_utf8 = (flags & 0x100) != 0;

        for i in 0..string_count {
            let string_offset: u32 = self.buffer.pread_with(offset + 28 + (i as usize * 4), LE).map_err(DexError::ScrollError)?;
            let mut current_offset = offset + strings_start as usize + string_offset as usize;

            if is_utf8 {
                let (_char_len, bytes_read_1) = self.decode_len(current_offset)?;
                current_offset += bytes_read_1;
                let (byte_len, bytes_read_2) = self.decode_len(current_offset)?;
                current_offset += bytes_read_2;

                if current_offset + byte_len > self.buffer.len() {
                    strings.push(String::new());
                    continue;
                }

                let s_bytes = &self.buffer[current_offset..current_offset + byte_len];
                strings.push(String::from_utf8_lossy(s_bytes).to_string());
            } else {
                let (char_len, bytes_read) = self.decode_len_u16(current_offset)?;
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

    fn decode_len(&self, offset: usize) -> Result<(usize, usize), DexError> {
        let first: u8 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if (first & 0x80) != 0 {
            let second: u8 = self.buffer.pread_with(offset + 1, LE).map_err(DexError::ScrollError)?;
            Ok(((((first & 0x7f) as usize) << 8) | (second as usize), 2))
        } else {
            Ok((first as usize, 1))
        }
    }

    fn decode_len_u16(&self, offset: usize) -> Result<(usize, usize), DexError> {
        let first: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if (first & 0x8000) != 0 {
            let second: u16 = self.buffer.pread_with(offset + 2, LE).map_err(DexError::ScrollError)?;
            Ok(((((first & 0x7fff) as usize) << 16) | (second as usize), 4))
        } else {
            Ok((first as usize, 2))
        }
    }
}
