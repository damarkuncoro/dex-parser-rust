use crate::dex::models::Field;

pub trait StringResolver {
    fn resolve_string(&self, idx: u32) -> Option<String>;
}

pub trait TypeResolver {
    fn resolve_type(&self, idx: u32) -> Option<String>;
}

pub trait MethodResolver {
    fn resolve_method(&self, idx: u32) -> Option<String>;
}

pub trait FieldResolver {
    fn resolve_field(&self, idx: u32) -> Option<Field>;
}

pub trait DexResolver: StringResolver + TypeResolver + MethodResolver + FieldResolver {}
