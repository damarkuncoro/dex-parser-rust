use scroll::{Endian, Pread};

pub fn extract_immediates(description: &str, op_unit: u16, units: &[u16], buffer: &[u8], pc: usize, endian: Endian) -> Vec<u64> {
    let mut imms = Vec::new();
    if description.contains("#+BBBBBBBB") {
        let b: u32 = buffer.pread_with(pc + 2, endian).unwrap_or(0);
        imms.push(b as u64);
    } else if description.contains("#+BBBB000000000000") {
        let b = units.get(1).cloned().unwrap_or(0) as u64;
        imms.push(b << 48);
    } else if description.contains("#+BBBB0000") {
        let b = units.get(1).cloned().unwrap_or(0) as u32;
        imms.push((b << 16) as u64);
    } else if description.contains("#+BBBB") {
        let b = units.get(1).cloned().unwrap_or(0) as i16;
        imms.push(b as i32 as u64);
    } else if description.contains("#+CC") {
        let cc = (units.get(1).cloned().unwrap_or(0) >> 8) as i8;
        imms.push(cc as i32 as u64);
    } else if description.contains("#+B") {
        let b = (op_unit >> 12) as i8;
        imms.push(b as i32 as u64);
    }
    imms
}

pub fn substitute_immediates(description: &mut String, op_unit: u16, units: &[u16], buffer: &[u8], pc: usize, endian: Endian) {
    if description.contains("#+BBBBBBBB") {
        let b: u32 = buffer.pread_with(pc + 2, endian).unwrap_or(0);
        *description = description.replace("#+BBBBBBBB", &format!("#0x{:08x}", b));
    } else if description.contains("#+BBBB000000000000") {
        let b = units.get(1).cloned().unwrap_or(0) as u64;
        *description = description.replace("#+BBBB000000000000", &format!("#0x{:016x}", b << 48));
    } else if description.contains("#+BBBB0000") {
        let b = units.get(1).cloned().unwrap_or(0) as u32;
        *description = description.replace("#+BBBB0000", &format!("#0x{:08x}", b << 16));
    } else if description.contains("#+BBBB") {
        let b = units.get(1).cloned().unwrap_or(0) as i16;
        *description = description.replace("#+BBBB", &format!("#{:+} (0x{:04x})", b, b as u16));
    } else if description.contains("#+CC") {
        let cc = (units.get(1).cloned().unwrap_or(0) >> 8) as i8;
        *description = description.replace("#+CC", &format!("#{}", cc));
    } else if description.contains("#+B") {
        let b = (op_unit >> 12) as i8;
        *description = description.replace("#+B", &format!("#{:+} (0x{:x})", b, b as u8 & 0xf));
    }
}
