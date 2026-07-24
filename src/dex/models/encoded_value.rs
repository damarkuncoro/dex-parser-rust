use serde::{Serialize};

#[derive(Serialize,  Clone, Debug)]
pub enum EncodedValue<'a> {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    MethodType(u32),
    MethodHandle(u32),
    String(&'a str),
    Type(&'a str),
    Field(String),
    Method(String),
    Enum(String),
    Array(Vec<EncodedValue<'a>>),
    Annotation(EncodedAnnotation<'a>),
    Null,
    Boolean(bool),
}

#[derive(Serialize,  Clone, Debug)]
pub struct EncodedAnnotation<'a> {
    pub type_name: &'a str,
    pub elements: Vec<AnnotationElement<'a>>,
}

#[derive(Serialize,  Clone, Debug)]
pub struct AnnotationElement<'a> {
    pub name: &'a str,
    pub value: EncodedValue<'a>,
}
