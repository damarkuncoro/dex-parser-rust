use crate::dex::core::models::{EncodedMethod};
use crate::exporter::core::ExportOptions;
use std::io::{Write};

pub fn export_method(method: &EncodedMethod, idx: usize, writer: &mut dyn Write, options: &ExportOptions) -> std::io::Result<()> {
    writeln!(writer, "    #{}              : (in ...)", idx)?;
    writeln!(writer, "      name          : '{}'", method.name)?;
    writeln!(writer, "      type          : '{}'", method.signature)?;
    writeln!(writer, "      access        : 0x{:04x} ({})", method.access_flags, method.access_flags_text)?;

    if let Some(code) = &method.code {
        writeln!(writer, "      code          -")?;
        writeln!(writer, "      registers     : {}", code.registers_size)?;
        writeln!(writer, "      ins           : {}", code.ins_size)?;
        writeln!(writer, "      outs          : {}", code.outs_size)?;
        writeln!(writer, "      insns size    : {} 16-bit code units", code.insns_size)?;

        if options.include_instructions {
            // Collect physical labels for branching
            let mut labels = std::collections::HashSet::new();
            for ins in &code.instructions {
                if ins.description.contains(":label_") {
                    if let Some(pos) = ins.description.find(":label_") {
                        let label_part = &ins.description[pos + 7..];
                        let end_pos = label_part.find(|c: char| !c.is_ascii_hexdigit()).unwrap_or(label_part.len());
                        let label = &label_part[..end_pos];
                        if let Ok(addr) = u32::from_str_radix(label, 16) {
                            labels.insert(addr);
                        }
                    }
                }
            }

            for ins in &code.instructions {
                if labels.contains(&(ins.offset as u32)) {
                    writeln!(writer, "      :label_{:04x}", ins.offset)?;
                }
                writeln!(writer, "      0x{:04x}: {} {}", ins.offset, ins.name, ins.description)?;
            }
        }

        if !code.catches.is_empty() {
            writeln!(writer, "      catches       : {}", code.catches.len())?;
            for (i, catch) in code.catches.iter().enumerate() {
                writeln!(writer, "        try #{} 0x{:04x} - 0x{:04x}", i, catch.start_addr, catch.end_addr)?;
                for handler in &catch.handlers {
                    writeln!(writer, "          catch {}; -> :label_{:04x}", handler.type_name, handler.addr)?;
                }
            }
        }

        if let Some(debug) = &code.debug_info {
            writeln!(writer, "      debug_info    -")?;
            writeln!(writer, "        line_start  : {}", debug.line_start)?;
            writeln!(writer, "        parameters  : {:?}", debug.parameters)?;
            for entry in &debug.entries {
                writeln!(writer, "        {:?}", entry)?;
            }
        }
    }

    Ok(())
}
