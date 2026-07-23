use crate::dex::models::{EncodedMethod, DebugEntry};
use std::io::Write;
use std::collections::{HashSet, HashMap};

pub fn print_method(idx: usize, method: &EncodedMethod, class_name: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "    #{:<14} : (in {})", idx, class_name)?;
    writeln!(writer, "      name          : '{}'", method.name)?;
    writeln!(writer, "      type          : '{}'", method.signature)?;
    writeln!(
        writer,
        "      access        : 0x{:04x} ({})",
        method.access_flags, method.access_flags_text
    )?;

    if let Some(code) = &method.code {
        writeln!(writer, "      code          -")?;
        writeln!(writer, "      registers     : {}", code.registers_size)?;
        writeln!(writer, "      ins           : {}", code.ins_size)?;
        writeln!(writer, "      outs          : {}", code.outs_size)?;
        writeln!(
            writer,
            "      insns size    : {} 16-bit code units",
            code.insns_size
        )?;

        // 1. Map Debug Entries by Address for inline printing
        let mut debug_map: HashMap<u32, Vec<&DebugEntry>> = HashMap::new();
        if let Some(debug) = &code.debug_info {
            for entry in &debug.entries {
                let addr = match entry {
                    DebugEntry::LineNumber { address_diff, .. } => *address_diff,
                    DebugEntry::StartLocal { address_diff, .. } => *address_diff,
                    DebugEntry::EndLocal { address_diff } => *address_diff,
                    DebugEntry::RestartLocal { address_diff } => *address_diff,
                };
                debug_map.entry(addr).or_default().push(entry);
            }
        }

        // 2. Identify all branch targets to print physical labels
        let mut labels = HashSet::new();
        for ins in &code.instructions {
            if let Some(pos) = ins.description.find(":label_") {
                if let Some(label_str) = ins.description.get(pos + 7..pos + 11) {
                    if let Ok(addr) = u32::from_str_radix(label_str, 16) {
                        labels.insert(addr);
                    }
                }
            }
        }

        for catch in &code.catches {
            for handler in &catch.handlers {
                labels.insert(handler.addr);
            }
        }

        // 3. Print instructions with integrated Debug Info & Labels
        for ins in &code.instructions {
            let addr = ins.offset as u32;

            // Inline Debug Info (Line numbers and Locals)
            if let Some(entries) = debug_map.get(&addr) {
                for entry in entries {
                    match entry {
                        DebugEntry::LineNumber { line_diff, .. } => {
                            writeln!(writer, "      .line {}", line_diff)?;
                        }
                        DebugEntry::StartLocal { name, type_name, .. } => {
                            writeln!(writer, "      .local \"{}\":{}", name, type_name)?;
                        }
                        DebugEntry::EndLocal { .. } => {
                            writeln!(writer, "      .end local")?;
                        }
                        DebugEntry::RestartLocal { .. } => {
                            writeln!(writer, "      .restart local")?;
                        }
                    }
                }
            }

            if labels.contains(&addr) {
                writeln!(writer, "      :label_{:04x}", addr)?;
            }
            writeln!(
                writer,
                "      0x{:04x}: {} {}",
                ins.offset, ins.name, ins.description
            )?;
        }

        if !code.catches.is_empty() {
            writeln!(writer, "      catches       : {}", code.catches.len())?;
            for (i, catch) in code.catches.iter().enumerate() {
                writeln!(
                    writer,
                    "        try #{} 0x{:04x} - 0x{:04x}",
                    i, catch.start_addr, catch.end_addr
                )?;
                for handler in &catch.handlers {
                    writeln!(writer, "          catch {} -> :label_{:04x}", handler.type_name, handler.addr)?;
                }
            }
        }
    } else {
        writeln!(writer, "      code          : (none)")?;
    }
    Ok(())
}
