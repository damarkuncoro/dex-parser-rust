use super::DexPrinter;
use crate::dex::models::{Class, Dex, EncodedField, EncodedMethod};

pub struct DexDumpPrinter;

impl DexPrinter for DexDumpPrinter {
    fn print(&self, dex: &Dex, path: &str) {
        println!("Processing '{}'...", path);
        println!("Opened '{}', DEX version '035'", path);
        self.print_header(dex);
        for (i, class) in dex.classes.iter().enumerate() {
            self.print_class(dex, i, class);
        }
    }
}

impl DexDumpPrinter {
    fn print_header(&self, dex: &Dex) {
        println!("DEX file header:");
        println!("magic               : 'dex\\n035\\0'");
        println!("checksum            : {:08x}", dex.header.checksum);
        let sig_hex = hex::encode(dex.header.signature);
        println!(
            "signature           : {}...{}",
            &sig_hex[..4],
            &sig_hex[sig_hex.len() - 4..]
        );
        println!("file_size           : {}", dex.header.file_size);
        println!("header_size         : {}", dex.header.header_size);
        println!("link_size           : {}", dex.header.link_size);
        println!(
            "link_off            : {} (0x{:06x})",
            dex.header.link_off, dex.header.link_off
        );
        println!("string_ids_size     : {}", dex.strings.len());
        println!(
            "string_ids_off      : {} (0x{:06x})",
            dex.header.string_ids_off, dex.header.string_ids_off
        );
        println!("type_ids_size       : {}", dex.types.len());
        println!(
            "type_ids_off        : {} (0x{:06x})",
            dex.header.type_ids_off, dex.header.type_ids_off
        );
        println!("proto_ids_size      : {}", dex.protos.len());
        println!(
            "proto_ids_off       : {} (0x{:06x})",
            dex.header.proto_ids_off, dex.header.proto_ids_off
        );
        println!("field_ids_size      : {}", dex.fields.len());
        println!(
            "field_ids_off       : {} (0x{:06x})",
            dex.header.field_ids_off, dex.header.field_ids_off
        );
        println!("method_ids_size     : {}", dex.methods.len());
        println!(
            "method_ids_off      : {} (0x{:06x})",
            dex.header.method_ids_off, dex.header.method_ids_off
        );
        println!("class_defs_size     : {}", dex.classes.len());
        println!(
            "class_defs_off      : {} (0x{:06x})",
            dex.header.class_defs_off, dex.header.class_defs_off
        );
        println!("data_size           : {}", dex.header.data_size);
        println!(
            "data_off            : {} (0x{:06x})",
            dex.header.data_off, dex.header.data_off
        );
    }

    fn print_class(&self, _dex: &Dex, index: usize, class: &Class) {
        println!("\nClass #{:<12} -", index);
        println!("  Class descriptor  : '{}'", class.name);
        println!(
            "  Access flags      : 0x{:04x} ({})",
            class.access_flags, class.access_flags_text
        );
        println!("  Superclass        : '{}'", class.superclass);

        println!("  Interfaces        -");
        for (i, itf) in class.interfaces.iter().enumerate() {
            println!("    #{:<14} : '{}'", i, itf);
        }

        println!("  Static fields     -");
        for (f_idx, field) in class.static_fields.iter().enumerate() {
            self.print_field(f_idx, field, &class.name);
        }

        println!("  Instance fields   -");
        for (f_idx, field) in class.instance_fields.iter().enumerate() {
            self.print_field(f_idx, field, &class.name);
        }

        println!("  Direct methods    -");
        for (m_idx, method) in class.direct_methods.iter().enumerate() {
            self.print_method(m_idx, method, &class.name);
        }
        println!("  Virtual methods   -");
        for (m_idx, method) in class.virtual_methods.iter().enumerate() {
            self.print_method(m_idx, method, &class.name);
        }

        let source_file = class
            .source_file
            .as_ref()
            .map(|s| format!("'{}'", s))
            .unwrap_or_else(|| "unknown".to_string());
        println!(
            "  source_file_idx   : {} ({})",
            class.source_file_idx, source_file
        );
    }

    fn print_field(&self, idx: usize, field: &EncodedField, class_name: &str) {
        println!("    #{:<14} : (in {})", idx, class_name);
        println!("      name          : '{}'", field.name);
        println!("      type          : '{}'", field.type_name);
        println!(
            "      access        : 0x{:04x} ({})",
            field.access_flags, field.access_flags_text
        );
    }

    fn print_method(&self, idx: usize, method: &EncodedMethod, class_name: &str) {
        println!("    #{:<14} : (in {})", idx, class_name);
        println!("      name          : '{}'", method.name);
        println!("      type          : '{}'", method.signature);
        println!(
            "      access        : 0x{:04x} ({})",
            method.access_flags, method.access_flags_text
        );
        if let Some(code) = &method.code {
            println!("      code          -");
            println!("      registers     : {}", code.registers_size);
            println!("      ins           : {}", code.ins_size);
            println!("      outs          : {}", code.outs_size);
            println!(
                "      insns size    : {} 16-bit code units",
                code.insns_size
            );

            for ins in &code.instructions {
                println!(
                    "      0x{:04x}: {} {}",
                    ins.offset, ins.name, ins.description
                );
            }

            if code.catches.is_empty() {
                println!("      catches       : (none)");
            } else {
                println!("      catches       : {}", code.catches.len());
                for catch in &code.catches {
                    println!(
                        "        0x{:04x} - 0x{:04x}",
                        catch.start_addr, catch.end_addr
                    );
                    for handler in &catch.handlers {
                        println!("          {} -> 0x{:04x}", handler.type_name, handler.addr);
                    }
                }
            }
            println!("      positions     :");
            println!("      locals        :");
        } else {
            println!("      code          : (none)");
        }
    }
}
