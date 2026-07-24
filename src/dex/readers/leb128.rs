use crate::dex::error::DexError;

/// Reads a ULEB128 value from the buffer starting at the given offset.
pub fn read_uleb128(buffer: &[u8], mut offset: usize) -> Result<(u64, usize), DexError> {
    let mut result = 0u64;
    let mut count = 0;
    let mut shift = 0;
    loop {
        if offset >= buffer.len() {
            return Err(DexError::UnexpectedEOF);
        }
        let byte = buffer[offset];
        offset += 1;
        count += 1;

        if shift >= 64 {
            return Err(DexError::MalformedULEB128(offset - count));
        }

        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok((result, count))
}

/// Reads an SLEB128 value from the buffer starting at the given offset.
pub fn read_sleb128(buffer: &[u8], mut offset: usize) -> Result<(i64, usize), DexError> {
    let mut result = 0i64;
    let mut count = 0;
    let mut shift = 0;
    let mut byte;
    loop {
        if offset >= buffer.len() {
            return Err(DexError::UnexpectedEOF);
        }
        byte = buffer[offset];
        offset += 1;
        count += 1;

        if shift >= 64 {
            return Err(DexError::MalformedULEB128(offset - count));
        }

        result |= ((byte & 0x7f) as i64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            break;
        }
    }
    if shift < 64 && (byte & 0x40) != 0 {
        result |= -(1 << shift);
    }
    Ok((result, count))
}
