use crate::dex::models::header::RawHeader;
use crate::dex::models::{Field, Proto};
use crate::dex::parsers::traits::{
    DexResolver, FieldResolver, MethodResolver, StringResolver, TypeResolver,
};
use scroll::Endian;

pub struct DexContext<'a> {
    pub buffer: &'a [u8],
    pub header: RawHeader,
    pub endian: Endian,

    pub strings: Vec<String>,
    pub types: Vec<String>,
    pub protos: Vec<Proto>,
    pub fields: Vec<Field>,
    pub methods: Vec<String>,
}

impl<'a> DexContext<'a> {
    pub fn new(buffer: &'a [u8], header: RawHeader, endian: Endian) -> Self {
        Self {
            buffer,
            header,
            endian,
            strings: Vec::new(),
            types: Vec::new(),
            protos: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

impl<'a> StringResolver for DexContext<'a> {
    fn resolve_string(&self, idx: u32) -> Option<String> {
        self.strings.get(idx as usize).cloned()
    }
}

impl<'a> TypeResolver for DexContext<'a> {
    fn resolve_type(&self, idx: u32) -> Option<String> {
        self.types.get(idx as usize).cloned()
    }
}

impl<'a> MethodResolver for DexContext<'a> {
    fn resolve_method(&self, idx: u32) -> Option<String> {
        self.methods.get(idx as usize).cloned()
    }
}

impl<'a> FieldResolver for DexContext<'a> {
    fn resolve_field(&self, idx: u32) -> Option<Field> {
        self.fields.get(idx as usize).cloned()
    }
}

impl<'a> DexResolver for DexContext<'a> {}
