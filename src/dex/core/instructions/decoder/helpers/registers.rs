pub fn extract_registers(description: &str, op_unit: u16, units: &[u16]) -> Vec<u16> {
    let mut regs = Vec::new();
    if description.contains("{vC..vG}") {
        let count = ((op_unit >> 12) & 0xf) as usize;
        let g = (op_unit >> 8) & 0xf;
        let regs_unit = units.get(2).cloned().unwrap_or(0);
        let c = regs_unit & 0xf;
        let d = (regs_unit >> 4) & 0xf;
        let e = (regs_unit >> 8) & 0xf;
        let f = (regs_unit >> 12) & 0xf;

        if count > 0 { regs.push(c); }
        if count > 1 { regs.push(d); }
        if count > 2 { regs.push(e); }
        if count > 3 { regs.push(f); }
        if count > 4 { regs.push(g); }
    } else if description.contains("{vCCCC..vNNNN}") {
        let count = (op_unit >> 8) & 0xff;
        let start = units.get(2).cloned().unwrap_or(0);
        for i in 0..count {
            regs.push(start.wrapping_add(i as u16));
        }
    } else {
        if description.contains("vAAAA") {
            regs.push(units.get(1).cloned().unwrap_or(0));
        } else {
            if description.contains("vAA") {
                regs.push((op_unit >> 8) & 0xff);
                if description.matches("vAA").count() > 1 || description.contains("vBB") {
                     if let Some(v) = units.get(1) { regs.push(v & 0xff); }
                }
            } else {
                if description.contains("vA") { regs.push((op_unit >> 8) & 0xf); }
                if description.contains("vB") { regs.push((op_unit >> 12) & 0xf); }
            }
        }
    }
    regs
}

pub fn substitute_registers(description: &mut String, op_unit: u16, units: &[u16]) {
    *description = description.replace("vAAAA", &format!("v{}", units.get(1).cloned().unwrap_or(0)));
    if description.contains("vBB") && description.contains("vCC") {
        let v_unit = units.get(1).cloned().unwrap_or(0);
        *description = description.replace("vBB", &format!("v{}", v_unit & 0xff));
        *description = description.replace("vCC", &format!("v{}", (v_unit >> 8) & 0xff));
    }
    if description.contains("vAA") && description.contains("vBB") {
         *description = description.replace("vAA", &format!("v{}", (op_unit >> 8) & 0xff));
         *description = description.replace("vBB", &format!("v{}", units.get(1).cloned().unwrap_or(0) & 0xff));
    }
    *description = description.replace("vAA", &format!("v{}", (op_unit >> 8) & 0xff));
    *description = description.replace("vA", &format!("v{}", (op_unit >> 8) & 0xf));
    *description = description.replace("vB", &format!("v{}", (op_unit >> 12) & 0xf));
}
