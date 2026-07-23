use crate::dex::models::{Class, Dex, EncodedField};
use super::method::print_method;

pub fn print_class(_dex: &Dex, index: usize, class: &Class) {
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
        print_field(f_idx, field, &class.name);
    }

    println!("  Instance fields   -");
    for (f_idx, field) in class.instance_fields.iter().enumerate() {
        print_field(f_idx, field, &class.name);
    }

    println!("  Direct methods    -");
    for (m_idx, method) in class.direct_methods.iter().enumerate() {
        print_method(m_idx, method, &class.name);
    }
    println!("  Virtual methods   -");
    for (m_idx, method) in class.virtual_methods.iter().enumerate() {
        print_method(m_idx, method, &class.name);
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

pub fn print_field(idx: usize, field: &EncodedField, class_name: &str) {
    println!("    #{:<14} : (in {})", idx, class_name);
    println!("      name          : '{}'", field.name);
    println!("      type          : '{}'", field.type_name);
    println!(
        "      access        : 0x{:04x} ({})",
        field.access_flags, field.access_flags_text
    );
}
