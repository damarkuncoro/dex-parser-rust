pub fn substitute_special(description: &mut String, op_unit: u16, units: &[u16]) {
    if description.contains("{vC..vG}") {
        let count = (op_unit >> 12) & 0xf;
        let g = (op_unit >> 8) & 0xf;
        let regs_unit = units.get(2).cloned().unwrap_or(0);
        let c = regs_unit & 0xf;
        let d = (regs_unit >> 4) & 0xf;
        let e = (regs_unit >> 8) & 0xf;
        let f = (regs_unit >> 12) & 0xf;

        let mut regs = Vec::new();
        if count > 0 { regs.push(format!("v{}", c)); }
        if count > 1 { regs.push(format!("v{}", d)); }
        if count > 2 { regs.push(format!("v{}", e)); }
        if count > 3 { regs.push(format!("v{}", f)); }
        if count > 4 { regs.push(format!("v{}", g)); }
        *description = description.replace("{vC..vG}", &format!("{{{}}}", regs.join(", ")));
    } else if description.contains("{vCCCC..vNNNN}") {
        let count = (op_unit >> 8) & 0xff;
        let start = units.get(2).cloned().unwrap_or(0);
        let end = if count > 0 { (start as u32 + count as u32 - 1) as u16 } else { start };
        *description = description.replace("{vCCCC..vNNNN}", &format!("{{v{} .. v{}}}", start, end));
    }
}
