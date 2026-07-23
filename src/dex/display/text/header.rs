use crate::dex::models::Dex;
use std::io::Write;

pub fn print_header(dex: &Dex, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "DEX file header:")?;
    writeln!(writer, "magic               : 'dex\\n035\\0'")?;
    writeln!(writer, "checksum            : {:08x}", dex.header.checksum)?;
    let sig_hex = hex::encode(dex.header.signature);
    writeln!(
        writer,
        "signature           : {}...{}",
        &sig_hex[..4],
        &sig_hex[sig_hex.len() - 4..]
    )?;
    writeln!(writer, "file_size           : {}", dex.header.file_size)?;
    writeln!(writer, "header_size         : {}", dex.header.header_size)?;
    writeln!(writer, "link_size           : {}", dex.header.link_size)?;
    writeln!(
        writer,
        "link_off            : {} (0x{:06x})",
        dex.header.link_off, dex.header.link_off
    )?;
    writeln!(writer, "string_ids_size     : {}", dex.metadata.strings.len())?;
    writeln!(
        writer,
        "string_ids_off      : {} (0x{:06x})",
        dex.header.string_ids_off, dex.header.string_ids_off
    )?;
    writeln!(writer, "type_ids_size       : {}", dex.metadata.types.len())?;
    writeln!(
        writer,
        "type_ids_off        : {} (0x{:06x})",
        dex.header.type_ids_off, dex.header.type_ids_off
    )?;
    writeln!(writer, "proto_ids_size      : {}", dex.metadata.protos.len())?;
    writeln!(
        writer,
        "proto_ids_off       : {} (0x{:06x})",
        dex.header.proto_ids_off, dex.header.proto_ids_off
    )?;
    writeln!(writer, "field_ids_size      : {}", dex.metadata.fields.len())?;
    writeln!(
        writer,
        "field_ids_off       : {} (0x{:06x})",
        dex.header.field_ids_off, dex.header.field_ids_off
    )?;
    writeln!(writer, "method_ids_size     : {}", dex.metadata.methods.len())?;
    writeln!(
        writer,
        "method_ids_off      : {} (0x{:06x})",
        dex.header.method_ids_off, dex.header.method_ids_off
    )?;
    writeln!(writer, "class_defs_size     : {}", dex.class_defs.len())?;
    writeln!(
        writer,
        "class_defs_off      : {} (0x{:06x})",
        dex.header.class_defs_off, dex.header.class_defs_off
    )?;
    writeln!(writer, "data_size           : {}", dex.header.data_size)?;
    writeln!(
        writer,
        "data_off            : {} (0x{:06x})",
        dex.header.data_off, dex.header.data_off
    )?;
    Ok(())
}

pub fn print_map_list(dex: &Dex, writer: &mut dyn Write) -> std::io::Result<()> {
    writeln!(writer, "Map list:")?;
    for (i, item) in dex.map_list.items.iter().enumerate() {
        writeln!(
            writer,
            "  #{:<2} type=0x{:04x} size={:<4} off=0x{:06x}",
            i, item.item_type, item.size, item.offset
        )?;
    }
    Ok(())
}
