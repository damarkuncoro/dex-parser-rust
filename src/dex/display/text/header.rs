use crate::dex::models::Dex;

pub fn print_header(dex: &Dex) {
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
    println!("string_ids_size     : {}", dex.metadata.strings.len());
    println!(
        "string_ids_off      : {} (0x{:06x})",
        dex.header.string_ids_off, dex.header.string_ids_off
    );
    println!("type_ids_size       : {}", dex.metadata.types.len());
    println!(
        "type_ids_off        : {} (0x{:06x})",
        dex.header.type_ids_off, dex.header.type_ids_off
    );
    println!("proto_ids_size      : {}", dex.metadata.protos.len());
    println!(
        "proto_ids_off       : {} (0x{:06x})",
        dex.header.proto_ids_off, dex.header.proto_ids_off
    );
    println!("field_ids_size      : {}", dex.metadata.fields.len());
    println!(
        "field_ids_off       : {} (0x{:06x})",
        dex.header.field_ids_off, dex.header.field_ids_off
    );
    println!("method_ids_size     : {}", dex.metadata.methods.len());
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

pub fn print_map_list(dex: &Dex) {
    println!("Map list:");
    for (i, item) in dex.map_list.items.iter().enumerate() {
        println!(
            "  #{:<2} type=0x{:04x} size={:<4} off=0x{:06x}",
            i, item.item_type, item.size, item.offset
        );
    }
}
