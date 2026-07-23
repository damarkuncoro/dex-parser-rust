use crate::dex::models::{Class, Dex, EncodedField};
use super::method::print_method;
use std::io::Write;

pub fn print_class(_dex: &Dex, index: usize, class: &Class, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "\nClass #{:<12} -", index)?;
    writeln!(writer, "  Class descriptor  : '{}'", class.name)?;
    writeln!(
        writer,
        "  Access flags      : 0x{:04x} ({})",
        class.access_flags, class.access_flags_text
    )?;
    writeln!(writer, "  Superclass        : '{}'", class.superclass)?;

    writeln!(writer, "  Interfaces        -")?;
    for (i, itf) in class.interfaces.iter().enumerate() {
        writeln!(writer, "    #{:<14} : '{}'", i, itf)?;
    }

    writeln!(writer, "  Static fields     -")?;
    for (f_idx, field) in class.static_fields.iter().enumerate() {
        print_field(f_idx, field, &class.name, writer)?;
    }

    writeln!(writer, "  Instance fields   -")?;
    for (f_idx, field) in class.instance_fields.iter().enumerate() {
        print_field(f_idx, field, &class.name, writer)?;
    }

    writeln!(writer, "  Direct methods    -")?;
    for (m_idx, method) in class.direct_methods.iter().enumerate() {
        print_method(m_idx, method, &class.name, writer)?;
    }
    writeln!(writer, "  Virtual methods   -")?;
    for (m_idx, method) in class.virtual_methods.iter().enumerate() {
        print_method(m_idx, method, &class.name, writer)?;
    }

    let source_file = class
        .source_file
        .as_ref()
        .map(|s| format!("'{}'", s))
        .unwrap_or_else(|| "unknown".to_string());
    writeln!(
        writer,
        "  source_file_idx   : {} ({})",
        class.source_file_idx, source_file
    )?;
    Ok(())
}

pub fn print_field(idx: usize, field: &EncodedField, class_name: &str, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "    #{:<14} : (in {})", idx, class_name)?;
    writeln!(writer, "      name          : '{}'", field.name)?;
    writeln!(writer, "      type          : '{}'", field.type_name)?;
    writeln!(
        writer,
        "      access        : 0x{:04x} ({})",
        field.access_flags, field.access_flags_text
    )?;
    Ok(())
}
