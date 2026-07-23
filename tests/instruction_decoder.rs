use dex_parser_rust::dex::instructions::InstructionDecoder;
use dex_parser_rust::dex::models::Field;
use dex_parser_rust::dex::parsers::traits::*;
use scroll::Endian;

struct MockResolver;

impl<'a> StringResolver<'a> for MockResolver {
    fn resolve_string(&self, idx: u32) -> Option<&'a str> {
        if idx == 1 {
            Some("HelloRust")
        } else {
            None
        }
    }
}

impl<'a> TypeResolver<'a> for MockResolver {
    fn resolve_type(&self, idx: u32) -> Option<&'a str> {
        if idx == 2 {
            Some("Ljava/lang/String;")
        } else {
            None
        }
    }
}

impl MethodResolver for MockResolver {
    fn resolve_method(&self, idx: u32) -> Option<String> {
        if idx == 3 {
            Some("Ljava/io/PrintStream;->println".to_string())
        } else {
            None
        }
    }
}

impl<'a> FieldResolver<'a> for MockResolver {
    fn resolve_field(&self, idx: u32) -> Option<Field<'a>> {
        if idx == 4 {
            Some(Field {
                class: "Ljava/lang/System;",
                name: "out",
                type_name: "Ljava/io/PrintStream;",
            })
        } else {
            None
        }
    }
}

impl<'a> DexResolver<'a> for MockResolver {}

#[test]
fn test_decode_const_string() {
    let resolver = MockResolver;
    let decoder = InstructionDecoder::new(&resolver);
    let buffer = [0x1a, 0x00, 0x01, 0x00]; // const-string v0, string@0001
    let (ins, len) = decoder.decode(&buffer, 0, 0, Endian::Little);

    assert_eq!(ins.name, "const-string");
    assert!(ins.description.contains("\"HelloRust\""));
    assert_eq!(len, 4);
}

#[test]
fn test_decode_sget() {
    let resolver = MockResolver;
    let decoder = InstructionDecoder::new(&resolver);
    let buffer = [0x62, 0x00, 0x04, 0x00]; // sget-object v0, field@0004
    let (ins, _) = decoder.decode(&buffer, 0, 0, Endian::Little);

    assert_eq!(ins.name, "sget-object");
    assert!(ins
        .description
        .contains("Ljava/lang/System;->out:Ljava/io/PrintStream;"));
}
