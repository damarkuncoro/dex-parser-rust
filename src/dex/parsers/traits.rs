use crate::dex::models::Field;

pub trait StringResolver<'a> {
    fn resolve_string(&self, idx: u32) -> Option<&'a str>;
}

pub trait TypeResolver<'a> {
    fn resolve_type(&self, idx: u32) -> Option<&'a str>;
}

pub trait MethodResolver {
    fn resolve_method(&self, idx: u32) -> Option<String>;
}

pub trait FieldResolver<'a> {
    fn resolve_field(&self, idx: u32) -> Option<Field<'a>>;
}

pub trait DexResolver<'a>: StringResolver<'a> + TypeResolver<'a> + MethodResolver + FieldResolver<'a> {}

pub struct SimpleResolver<'a> {
    pub strings: Vec<&'a str>,
    pub types: Vec<&'a str>,
    pub methods: Vec<String>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> StringResolver<'a> for SimpleResolver<'a> {
    fn resolve_string(&self, idx: u32) -> Option<&'a str> {
        self.strings.get(idx as usize).copied()
    }
}

impl<'a> TypeResolver<'a> for SimpleResolver<'a> {
    fn resolve_type(&self, idx: u32) -> Option<&'a str> {
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
