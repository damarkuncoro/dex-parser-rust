use dex_parser_rust::dex::readers::DexReader;
use scroll::Endian;

#[test]
fn test_malformed_uleb128_overflow() {
    // 10 bytes with MSB set (0x80) will trigger shift >= 64
    let buffer = [0x80; 12];
    let mut reader = DexReader::new(&buffer, Endian::Little);

    let result = reader.read_uleb128();
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Malformed ULEB128"));
}

#[test]
fn test_unexpected_eof_leb128() {
    // 0x80 indicates more bytes follow, but buffer ends
    let buffer = [0x80];
    let mut reader = DexReader::new(&buffer, Endian::Little);

    let result = reader.read_uleb128();
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Unexpected end of file"));
}
