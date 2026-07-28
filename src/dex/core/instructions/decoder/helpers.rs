use crate::dex::core::instructions::opcodes::{IndexType};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::core::utils::mutf8::Mutf8Display;
use scroll::{Endian, Pread};

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
        // vAAAA
        if description.contains("vAAAA") {
            regs.push(units.get(1).cloned().unwrap_or(0));
        } else {
            // vAA
            if description.contains("vAA") {
                regs.push((op_unit >> 8) & 0xff);
                if description.matches("vAA").count() > 1 || description.contains("vBB") {
                     if let Some(v) = units.get(1) { regs.push(v & 0xff); }
                }
            } else {
                // vA, vB, vC
                if description.contains("vA") { regs.push((op_unit >> 8) & 0xf); }
                if description.contains("vB") { regs.push((op_unit >> 12) & 0xf); }
            }
        }
    }
    regs
}

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

pub fn resolve_xref<'a, R: DexResolver<'a>>(
    description: &mut String,
    index_type: IndexType,
    index: u32,
    resolver: &R,
) -> String {
    let resolved = match index_type {
        IndexType::String => resolver.resolve_string(index)
            .map(|b| format!("\"{}\"", Mutf8Display(b)))
            .unwrap_or_else(|| format!("string@{:04x}", index)),
        IndexType::Type => resolver.resolve_type(index)
            .map(|b| format!("{}", Mutf8Display(b)))
            .unwrap_or_else(|| format!("type@{:04x}", index)),
        IndexType::Method => resolver.resolve_method(index)
            .unwrap_or_else(|| format!("meth@{:04x}", index)),
        IndexType::Field => resolver.resolve_field(index)
            .map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name))
            .unwrap_or_else(|| format!("field@{:04x}", index)),
        IndexType::CallSite => format!("call_site@{:04x}", index),
        IndexType::MethodHandle => format!("method_handle@{:04x}", index),
        IndexType::Proto => format!("proto@{:04x}", index),
        IndexType::None => String::new(),
    };

    if description.contains("string@") { *description = description.replace("string@", &format!("{}", resolved)); }
    else if description.contains("type@") { *description = description.replace("type@", &format!("{}", resolved)); }
    else if description.contains("meth@") { *description = description.replace("meth@", &format!("{}", resolved)); }
    else if description.contains("field@") { *description = description.replace("field@", &format!("{}", resolved)); }
    else if description.contains("call_site@") { *description = description.replace("call_site@", &format!("{}", resolved)); }
    else if description.contains("method_handle@") { *description = description.replace("method_handle@", &format!("{}", resolved)); }
    else if description.contains("proto@") { *description = description.replace("proto@", &format!("{}", resolved)); }
    else if index_type != IndexType::None { *description = format!("{} {}", description, resolved); }

    resolved
}
