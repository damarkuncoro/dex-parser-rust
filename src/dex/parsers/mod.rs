pub mod header;
pub mod strings;
pub mod types;
pub mod protos;
pub mod fields;
pub mod methods;
pub mod classes;
pub mod class_data;
pub mod code;
pub mod traits;
pub mod encoded_value;
pub mod annotations;
pub mod debug_info;
pub mod map_list;

use crate::dex::constants::{offsets::ENDIAN_TAG, ENDIAN_CONSTANT};
use crate::dex::error::DexError;
use crate::dex::models::{Dex, DexMetadata};
use crate::dex::readers::DexReader;
use crate::dex::validator::DexValidator;
use crate::dex::linker::DexLinker;
use self::header::HeaderParser;
use self::strings::StringSection;
use self::types::TypeIdParser;
use self::methods::MethodIdParser;
use self::protos::ProtoIdParser;
use self::fields::FieldIdParser;
use self::classes::ClassDefParser;
use self::traits::SimpleResolver;
use scroll::Endian;
use std::io::Read;

pub struct DexParser<'a> {
    buffer: &'a [u8],
}

impl<'a> DexParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    /// Primary entry point: Parse DEX from an in-memory buffer.
    pub fn parse(buffer: &'a [u8]) -> Result<Dex<'a>, DexError> {
        Self::new(buffer).parse_internal()
    }

    fn parse_internal(self) -> Result<Dex<'a>, DexError> {
        let endian = self.detect_endian()?;
        let mut reader = DexReader::new(self.buffer, endian);

        // Stage 1: Atomic Extraction (Physical)
        let header = HeaderParser::parse(&mut reader)?;
        DexValidator::new().validate(self.buffer, &header)?;

        let string_offsets = StringSection::parse_offsets(&mut reader, header.string_ids_size, header.string_ids_off)?;
        let type_indices = TypeIdParser::parse(&mut reader, header.type_ids_size, header.type_ids_off)?;
        let raw_protos = ProtoIdParser::parse(&mut reader, header.proto_ids_size, header.proto_ids_off)?;
        let raw_fields = FieldIdParser::parse(&mut reader, header.field_ids_size, header.field_ids_off)?;
        let raw_methods = MethodIdParser::parse(&mut reader, header.method_ids_size, header.method_ids_off)?;
        let raw_classes = ClassDefParser::parse(&mut reader, header.class_defs_size, header.class_defs_off)?;

        // Stage 2: Value Resolution (Zero-Copy)
        let strings = StringSection::resolve_strings(self.buffer, &string_offsets)?;

        // Stage 3: Logical Linking (Application)
        let resolved_types: Vec<&'a str> = type_indices.iter()
            .map(|&idx| strings.get(idx as usize).copied().unwrap_or("<invalid>"))
            .collect();

        let mut protos = DexLinker::link_protos(&raw_protos, &strings, &resolved_types);
        for (i, raw) in raw_protos.iter().enumerate() {
            if raw.parameters_off != 0 {
                let mut p_reader = DexReader::new(self.buffer, endian);
                p_reader.seek(raw.parameters_off as usize)?;
                let size = p_reader.read_u32()?;
                for _ in 0..size {
                    let type_idx = p_reader.read_u16()?;
                    if let Some(t) = resolved_types.get(type_idx as usize) {
                        protos[i].parameters.push(t);
                    }
                }
            }
        }

        let fields = DexLinker::link_fields(&raw_fields, &strings, &resolved_types);
        let methods_display = DexLinker::link_methods(&raw_methods, &strings, &resolved_types, &protos);

        // Stage 4: High-Level Class Assembly (Parallel)
        let resolver = SimpleResolver {
            strings: strings.clone(),
            types: resolved_types.clone(),
            methods: methods_display.clone(),
            fields: fields.clone(),
        };

        let classes = classes::linker::parse_linked(
            self.buffer,
            &header,
            &strings,
            &resolved_types,
            &fields,
            &methods_display,
            &raw_classes,
            &raw_methods,
            &resolver,
            endian
        )?;

        let map_list = map_list::parse(self.buffer, header.map_off as usize, endian)?;

        Ok(Dex {
            header,
            metadata: DexMetadata {
                strings,
                types: resolved_types,
                protos,
                fields,
                methods: methods_display,
            },
            class_defs: classes,
            map_list,
        })
    }

    fn detect_endian(&self) -> Result<Endian, DexError> {
        use scroll::Pread;
        let tag: u32 = self.buffer.pread_with(ENDIAN_TAG, Endian::Little).map_err(DexError::ScrollError)?;
        if tag == ENDIAN_CONSTANT { Ok(Endian::Little) } else { Ok(Endian::Big) }
    }
}

/// Standalone convenience functions for the Public API
impl DexParser<'static> {
    /// Convenience: Parse DEX directly from a file path.
    /// Note: This reads and leaks the buffer to provide a 'static lifetime.
    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<Dex<'static>, DexError> {
        let mut file = std::fs::File::open(path).map_err(DexError::IoError)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(DexError::IoError)?;
        let leaked: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        DexParser::parse(leaked)
    }
}
