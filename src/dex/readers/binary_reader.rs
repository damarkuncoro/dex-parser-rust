use crate::dex::error::DexError;
use crate::dex::readers::leb128;
use scroll::{Endian, Pread};

/// A stateful reader for binary DEX data.
pub struct DexReader<'a> {
    buffer: &'a [u8],
    pos: usize,
    endian: Endian,
}

impl<'a> DexReader<'a> {
    /// Creates a new `DexReader` with a default endianness.
    pub fn new(buffer: &'a [u8], endian: Endian) -> Self {
        Self {
            buffer,
            pos: 0,
            endian,
        }
    }

    /// Reads a single byte and advances the position.
    pub fn read_u8(&mut self) -> Result<u8, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.pos += 1;
        Ok(val)
    }

    /// Reads a 16-bit unsigned integer and advances the position.
    pub fn read_u16(&mut self) -> Result<u16, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.pos += 2;
        Ok(val)
    }

    /// Reads a 32-bit unsigned integer and advances the position.
    pub fn read_u32(&mut self) -> Result<u32, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.pos += 4;
        Ok(val)
    }

    /// Reads a 32-bit signed integer and advances the position.
    pub fn read_i32(&mut self) -> Result<i32, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.pos += 4;
        Ok(val)
    }

    /// Returns a slice of the given length and advances the position (zero-copy).
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], DexError> {
        if self.pos + len > self.buffer.len() {
            return Err(DexError::UnexpectedEOF);
        }
        let bytes = &self.buffer[self.pos..self.pos + len];
        self.pos += len;
        Ok(bytes)
    }

    /// Reads a ULEB128 value and advances the position.
    pub fn read_uleb128(&mut self) -> Result<u64, DexError> {
        let (val, count) = leb128::read_uleb128(self.buffer, self.pos)?;
        self.pos += count;
        Ok(val)
    }

    /// Reads an SLEB128 value and advances the position.
    pub fn read_sleb128(&mut self) -> Result<i64, DexError> {
        let (val, count) = leb128::read_sleb128(self.buffer, self.pos)?;
        self.pos += count;
        Ok(val)
    }

    /// Sets the current reading position.
    pub fn seek(&mut self, pos: usize) -> Result<(), DexError> {
        if pos > self.buffer.len() {
            return Err(DexError::InvalidOffset(format!("Seek position {} out of bounds", pos)));
        }
        self.pos = pos;
        Ok(())
    }

    /// Returns the current reading position.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Returns the current endianness.
    pub fn endian(&self) -> Endian {
        self.endian
    }

    /// Sets the endianness for subsequent read operations.
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    /// Peek at the next byte without advancing the position.
    pub fn peek_u8(&self) -> Result<u8, DexError> {
        self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)
    }
}
