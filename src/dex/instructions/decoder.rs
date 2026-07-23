use crate::dex::instructions::opcodes::{IndexType, OpcodeTable};
use crate::dex::models::Instruction;
use crate::dex::parsers::traits::DexResolver;
use scroll::{Endian, Pread};

/// Decodes raw Dalvik bytecode into high-level `Instruction` structures.
///
/// It uses a `DexResolver` to resolve constant pool indexes (strings, types, methods, fields)
/// into human-readable strings during the decoding process.
pub struct InstructionDecoder<'a, R: DexResolver> {
    resolver: &'a R,
}

impl<'a, R: DexResolver> InstructionDecoder<'a, R> {
    /// Creates a new decoder with the given resolver.
    pub fn new(resolver: &'a R) -> Self {
        Self { resolver }
    }

    /// Decodes a single instruction at the given program counter (pc).
    ///
    /// Returns the decoded `Instruction` and the number of bytes consumed.
    pub fn decode(
        &self,
        buffer: &[u8],
        pc: usize,
        curr: usize,
        endian: Endian,
    ) -> (Instruction, usize) {
        let opcode_byte: u8 = buffer.pread_with(pc, endian).unwrap_or(0);
        let info = OpcodeTable::get(opcode_byte);

        let mut description = info.format.clone();

        if info.index_type != IndexType::None {
            let index: u32 = if opcode_byte == 0x1b {
                // const-string/jumbo
                buffer.pread_with(pc + 2, endian).unwrap_or(0)
            } else {
                let idx16: u16 = buffer.pread_with(pc + 2, endian).unwrap_or(0);
                idx16 as u32
            };

            let resolved = match info.index_type {
                IndexType::String => self
                    .resolver
                    .resolve_string(index)
                    .map(|s| format!("\"{}\"", s))
                    .unwrap_or_else(|| format!("string@{:04x}", index)),
                IndexType::Type => self
                    .resolver
                    .resolve_type(index)
                    .unwrap_or_else(|| format!("type@{:04x}", index)),
                IndexType::Method => self
                    .resolver
                    .resolve_method(index)
                    .unwrap_or_else(|| format!("meth@{:04x}", index)),
                IndexType::Field => self
                    .resolver
                    .resolve_field(index)
                    .map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name))
                    .unwrap_or_else(|| format!("field@{:04x}", index)),
                IndexType::None => String::new(),
            };
            description = format!("{}{}", info.format, resolved);
        }

        let instruction = Instruction {
            offset: pc - curr,
            opcode: opcode_byte,
            name: info.name,
            description,
        };

        (instruction, info.length * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex::models::Field;
    use crate::dex::parsers::traits::*;

    struct MockResolver;
    impl StringResolver for MockResolver {
        fn resolve_string(&self, idx: u32) -> Option<String> {
            if idx == 1 {
                Some("HelloRust".to_string())
            } else {
                None
            }
        }
    }
    impl TypeResolver for MockResolver {
        fn resolve_type(&self, idx: u32) -> Option<String> {
            if idx == 2 {
                Some("Ljava/lang/String;".to_string())
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
    impl FieldResolver for MockResolver {
        fn resolve_field(&self, idx: u32) -> Option<Field> {
            if idx == 4 {
                Some(Field {
                    class: "Ljava/lang/System;".to_string(),
                    name: "out".to_string(),
                    type_name: "Ljava/io/PrintStream;".to_string(),
                })
            } else {
                None
            }
        }
    }
    impl DexResolver for MockResolver {}

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
        let buffer = [0x62, 0x00, 0x04, 0x00]; // sget v0, field@0004
        let (ins, _) = decoder.decode(&buffer, 0, 0, Endian::Little);

        assert_eq!(ins.name, "sget");
        assert!(ins
            .description
            .contains("Ljava/lang/System;->out:Ljava/io/PrintStream;"));
    }
}
