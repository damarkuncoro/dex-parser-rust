use crate::dex::models::EncodedMethod;

pub fn print_method(idx: usize, method: &EncodedMethod, class_name: &str) {
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
