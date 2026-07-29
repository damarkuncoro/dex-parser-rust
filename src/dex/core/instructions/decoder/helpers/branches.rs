pub fn extract_branch_target(description: &str, op_unit: u16, units: &[u16]) -> Option<i32> {
    if description.contains("+CCCC") {
        Some(units.get(1).cloned().unwrap_or(0) as i16 as i32)
    } else if description.contains("+BBBB") {
        Some(units.get(1).cloned().unwrap_or(0) as i16 as i32)
    } else if description.contains("+AA") {
        Some(((op_unit >> 8) & 0xff) as i8 as i32)
    } else {
        None
    }
}

pub fn substitute_branches(description: &mut String, op_unit: u16, units: &[u16], current_instr_byte_addr: usize) {
    if description.contains("+CCCC") {
        let off = units.get(1).cloned().unwrap_or(0) as i16;
        *description = description.replace("+CCCC", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
    } else if description.contains("+BBBB") {
        let off = units.get(1).cloned().unwrap_or(0) as i16;
        *description = description.replace("+BBBB", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
    } else if description.contains("+AA") {
        let off = ((op_unit >> 8) & 0xff) as i8;
        *description = description.replace("+AA", &format!(":label_{:04x}", (current_instr_byte_addr as i32 + (off as i32 * 2)) as u32));
    }
}
