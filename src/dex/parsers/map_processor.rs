use crate::dex::core::models::map_list::{types as map_types, MapList};
use crate::dex::readers::DexReader;
use crate::dex::error::DexError;
use crate::dex::parsers::{
    strings::StringSection, types::TypeIdParser, protos::ProtoIdParser,
    fields::FieldIdParser, methods::MethodIdParser, classes::ClassDefParser,
    method_handles::MethodHandleParser, call_sites::CallSiteParser,
};
use crate::dex::core::models::raw::{RawMethodId, RawProtoId, RawFieldId, RawClassDef, RawMethodHandleItem, RawCallSiteIdItem};

pub struct MapProcessorResults {
    pub string_offsets: Vec<u32>,
    pub type_indices: Vec<u32>,
    pub raw_protos: Vec<RawProtoId>,
    pub raw_fields: Vec<RawFieldId>,
    pub raw_methods: Vec<RawMethodId>,
    pub raw_classes: Vec<RawClassDef>,
    pub method_handles: Vec<RawMethodHandleItem>,
    pub call_sites: Vec<RawCallSiteIdItem>,
}

pub struct MapProcessor;

impl MapProcessor {
    pub fn process(reader: &mut DexReader, map_list: &MapList) -> Result<MapProcessorResults, DexError> {
        let mut results = MapProcessorResults {
            string_offsets: Vec::new(),
            type_indices: Vec::new(),
            raw_protos: Vec::new(),
            raw_fields: Vec::new(),
            raw_methods: Vec::new(),
            raw_classes: Vec::new(),
            method_handles: Vec::new(),
            call_sites: Vec::new(),
        };

        for item in &map_list.items {
            match item.item_type {
                map_types::TYPE_STRING_ID_ITEM => {
                    results.string_offsets = StringSection::parse_offsets(reader, item.size, item.offset)?;
                }
                map_types::TYPE_TYPE_ID_ITEM => {
                    results.type_indices = TypeIdParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_PROTO_ID_ITEM => {
                    results.raw_protos = ProtoIdParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_FIELD_ID_ITEM => {
                    results.raw_fields = FieldIdParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_METHOD_ID_ITEM => {
                    results.raw_methods = MethodIdParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_CLASS_DEF_ITEM => {
                    results.raw_classes = ClassDefParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_METHOD_HANDLE_ITEM => {
                    results.method_handles = MethodHandleParser::parse(reader, item.size, item.offset)?;
                }
                map_types::TYPE_CALL_SITE_ID_ITEM => {
                    results.call_sites = CallSiteParser::parse(reader, item.size, item.offset)?;
                }
                _ => {}
            }
        }

        Ok(results)
    }
}
