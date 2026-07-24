use crate::dex::core::models::Field;

pub trait StringResolver<'a> {
    fn resolve_string(&self, idx: u32) -> Option<&'a [u8]>;
}

pub trait TypeResolver<'a> {
    fn resolve_type(&self, idx: u32) -> Option<&'a [u8]>;
}

pub trait MethodResolver {
    fn resolve_method(&self, idx: u32) -> Option<String>;
}

pub trait FieldResolver<'a> {
    fn resolve_field(&self, idx: u32) -> Option<Field<'a>>;
}

pub trait DexResolver<'a>: StringResolver<'a> + TypeResolver<'a> + MethodResolver + FieldResolver<'a> {}

pub struct SimpleResolver<'a> {
    pub strings: Vec<&'a [u8]>,
    pub types: Vec<&'a [u8]>,
    pub methods: Vec<String>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> StringResolver<'a> for SimpleResolver<'a> {
    fn resolve_string(&self, idx: u32) -> Option<&'a [u8]> {
        self.strings.get(idx as usize).copied()
    }
}

impl<'a> TypeResolver<'a> for SimpleResolver<'a> {
    fn resolve_type(&self, idx: u32) -> Option<&'a [u8]> {
        self.types.get(idx as usize).copied()
    }
}

impl<'a> MethodResolver for SimpleResolver<'a> {
    fn resolve_method(&self, idx: u32) -> Option<String> {
        self.methods.get(idx as usize).cloned()
    }
}

impl<'a> FieldResolver<'a> for SimpleResolver<'a> {
    fn resolve_field(&self, idx: u32) -> Option<Field<'a>> {
        self.fields.get(idx as usize).cloned()
    }
}

impl<'a> DexResolver<'a> for SimpleResolver<'a> {}
