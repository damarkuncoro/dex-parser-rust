use crate::dex::error::DexError;

pub fn read_int(buffer: &[u8], curr: &mut usize, size: usize) -> Result<i64, DexError> {
    let mut val: i64 = 0;
    for i in 0..size {
        let b = buffer.get(*curr + i).ok_or(DexError::UnexpectedEOF)?;
        val |= (*b as i64) << (i * 8);
    }
    *curr += size;
    // Sign extend
    let shift = (8 - size) * 8;
    Ok((val << shift) >> shift)
}

pub fn read_uint(buffer: &[u8], curr: &mut usize, size: usize) -> Result<u64, DexError> {
    let mut val: u64 = 0;
    for i in 0..size {
        let b = buffer.get(*curr + i).ok_or(DexError::UnexpectedEOF)?;
        val |= (*b as u64) << (i * 8);
    }
    *curr += size;
    Ok(val)
}
