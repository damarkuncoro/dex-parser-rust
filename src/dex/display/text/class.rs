use crate::dex::core::models::{Class, Dex, EncodedField};
use crate::dex::display::text::method::print_method;
use std::io::{Write};

pub fn print_class(class: &Class, idx: usize, _dex: &Dex, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "\nClass #{}            -", idx)?;
    writeln!(writer, "  Class descriptor  : '{}'", class.name)?;
    writeln!(writer, "  Access flags      : 0x{:04x} ({})", class.access_flags, class.access_flags_text)?;
    writeln!(writer, "  Superclass        : '{}'", class.superclass)?;

    if !class.interfaces.is_empty() {
        writeln!(writer, "  Interfaces        -")?;
        for (i, itf) in class.interfaces.iter().enumerate() {
            writeln!(writer, "    #{}              : '{}'", i, itf)?;
        }
    } else {
        writeln!(writer, "  Interfaces        -")?;
    }

    writeln!(writer, "  Static fields     -")?;
    print_fields(&class.static_fields, writer)?;

    writeln!(writer, "  Instance fields   -")?;
    print_fields(&class.instance_fields, writer)?;

    writeln!(writer, "  Direct methods    -")?;
    for (m_idx, method) in class.direct_methods.iter().enumerate() {
        print_method(method, m_idx, writer)?;
    }

    writeln!(writer, "  Virtual methods   -")?;
    for (m_idx, method) in class.virtual_methods.iter().enumerate() {
        print_method(method, m_idx, writer)?;
    }

    writeln!(writer, "  source_file_idx   : {}", class.source_file_idx)?;
    if let Some(sf) = &class.source_file {
        writeln!(writer, "  source_file       : '{}'", sf)?;
    }

    Ok(())
}

fn print_fields(fields: &[EncodedField], writer: &mut dyn Write) -> std::io::Result<()> {
    for (f_idx, field) in fields.iter().enumerate() {
        writeln!(writer, "    #{}              : (in ...)", f_idx)?;
        writeln!(writer, "      name          : '{}'", field.name)?;
        writeln!(writer, "      type          : '{}'", field.type_name)?;
        writeln!(writer, "      access        : 0x{:04x} ({})", field.access_flags, field.access_flags_text)?;
    }
    Ok(())
}
