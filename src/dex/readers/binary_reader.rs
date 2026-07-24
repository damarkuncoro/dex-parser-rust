use crate::dex::error::DexError;
use crate::dex::readers::leb128;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use scroll::{Endian, Pread};
use std::sync::{Arc, Mutex};

/// A stateful reader for binary DEX data.
pub struct DexReader<'a> {
    buffer: &'a [u8],
    pos: usize,
    endian: Endian,
    tracker: Option<Arc<Mutex<ByteTracker>>>,
}

impl<'a> DexReader<'a> {
    /// Creates a new `DexReader` with a default endianness.
    pub fn new(buffer: &'a [u8], endian: Endian) -> Self {
        Self {
            buffer,
            pos: 0,
            endian,
            tracker: None,
        }
    }

    pub fn with_tracker(mut self, tracker: Arc<Mutex<ByteTracker>>) -> Self {
        self.tracker = Some(tracker);
        self
    }

    fn mark_usage(&self, len: usize) {
        if let Some(t) = &self.tracker {
            if let Ok(mut tracker) = t.lock() {
                tracker.mark(self.pos, len);
            }
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.mark_usage(1);
        self.pos += 1;
        Ok(val)
    }

    pub fn read_u16(&mut self) -> Result<u16, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.mark_usage(2);
        self.pos += 2;
        Ok(val)
    }

    pub fn read_u32(&mut self) -> Result<u32, DexError> {
        let val = self.buffer.pread_with(self.pos, self.endian).map_err(DexError::ScrollError)?;
        self.mark_usage(4);
        self.pos += 4;
        Ok(val)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], DexError> {
        if self.pos + len > self.buffer.len() {
            return Err(DexError::UnexpectedEOF);
        }
        let bytes = &self.buffer[self.pos..self.pos + len];
        self.mark_usage(len);
        self.pos += len;
        Ok(bytes)
    }

    pub fn read_uleb128(&mut self) -> Result<u64, DexError> {
        let (val, count) = leb128::read_uleb128(self.buffer, self.pos)?;
        self.mark_usage(count);
        self.pos += count;
        Ok(val)
    }

    pub fn read_sleb128(&mut self) -> Result<i64, DexError> {
        let (val, count) = leb128::read_sleb128(self.buffer, self.pos)?;
        self.mark_usage(count);
        self.pos += count;
        Ok(val)
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), DexError> {
        if pos > self.buffer.len() {
            return Err(DexError::InvalidOffset(format!("Seek position {} out of bounds", pos)));
        }
        self.pos = pos;
        Ok(())
    }

    pub fn position(&self) -> usize { self.pos }
    pub fn buffer(&self) -> &'a [u8] { self.buffer }
    pub fn endian(&self) -> Endian { self.endian }
    pub fn set_endian(&mut self, endian: Endian) { self.endian = endian; }
}
