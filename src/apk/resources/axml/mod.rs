pub mod string_pool;
pub mod element;

use scroll::{Pread, LE};
use crate::dex::error::DexError;
use crate::dex::core::models::Manifest;
pub use string_pool::StringPoolDecoder;
pub use element::ElementProcessor;

pub struct AxmlParser<'a> {
    buffer: &'a [u8],
}

impl<'a> AxmlParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn parse_manifest(&self) -> Result<Manifest, DexError> {
        let mut manifest = Manifest::default();
        let mut offset = 0;

        let magic: u32 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
        if magic != 0x00080003 {
            eprintln!("      [!] AXML Invalid Magic: 0x{:08x}", magic);
            return Err(DexError::InvalidMagic);
        }
        offset += 8;

        let string_pool_decoder = StringPoolDecoder::new(self.buffer);
        let element_processor = ElementProcessor::new(self.buffer);

        let mut string_pool = Vec::new();
        let mut element_stack = Vec::new();

        while offset < self.buffer.len() {
            let chunk_type: u16 = self.buffer.pread_with(offset, LE).map_err(DexError::ScrollError)?;
            let chunk_size: u32 = self.buffer.pread_with(offset + 4, LE).map_err(DexError::ScrollError)?;

            match chunk_type {
                0x0001 => { // String Pool
                    string_pool = string_pool_decoder.decode(offset)?;
                    // eprintln!("      [D] AXML String Pool size: {}", string_pool.len());
                }
                0x0102 => { // Start Element
                    let name_idx: u32 = self.buffer.pread_with(offset + 20, LE).unwrap_or(0);
                    let name = string_pool.get(name_idx as usize).cloned().unwrap_or_default();
                    element_stack.push(name);
                    element_processor.process(offset, &string_pool, &mut manifest, &element_stack)?;
                }
                0x0103 => { // End Element
                    element_stack.pop();
                }
                _ => {}
            }

            if chunk_size == 0 { break; }
            offset += chunk_size as usize;
        }

        Ok(manifest)
    }
}
