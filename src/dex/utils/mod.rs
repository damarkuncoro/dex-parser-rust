pub mod access_flags;

pub fn calculate_adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

pub fn read_uleb128(buffer: &[u8], mut offset: usize) -> (u64, usize) {
    let mut result = 0u64;
    let mut count = 0;
    let mut shift = 0;
    loop {
        if offset >= buffer.len() {
            break;
        }
        let byte = buffer[offset];
        offset += 1;
        count += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    (result, count)
}

pub fn read_sleb128(buffer: &[u8], mut offset: usize) -> (i64, usize) {
    let mut result = 0i64;
    let mut count = 0;
    let mut shift = 0;
    let mut byte;
    loop {
        byte = buffer[offset];
        offset += 1;
        count += 1;
        result |= ((byte & 0x7f) as i64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            break;
        }
    }
    if shift < 64 && (byte & 0x40) != 0 {
        result |= -(1 << shift);
    }
    (result, count)
}
