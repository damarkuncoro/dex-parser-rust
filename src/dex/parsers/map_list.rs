use scroll::{Endian, Pread};
use crate::dex::error::DexError;
use crate::dex::models::{MapList, MapItem};

pub fn parse(buffer: &[u8], offset: usize, endian: Endian) -> Result<MapList, DexError> {
    if offset == 0 {
        return Ok(MapList { items: Vec::new() });
    }

    let mut curr = offset;
    let size: u32 = buffer.pread_with(curr, endian)?;
    curr += 4;

    let mut items = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let item: MapItem = buffer.pread_with(curr, endian)?;
        curr += 12; // MapItem size is 12 bytes
        items.push(item);
    }

    Ok(MapList { items })
}
