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
    String(String),
    Type(String),
    Field(String),
    Method(String),
    Enum(String),
    Array(Vec<EncodedValue<'a>>),
    Annotation(EncodedAnnotation<'a>),
    Null,
    Boolean(bool),
    #[serde(skip)] _Marker(std::marker::PhantomData<&'a ()>),
}

#[derive(Serialize,  Clone, Debug)]
pub struct EncodedAnnotation<'a> {
    pub type_name: String,
    pub elements: Vec<AnnotationElement<'a>>,
}

#[derive(Serialize,  Clone, Debug)]
pub struct AnnotationElement<'a> {
    pub name: String,
    pub value: EncodedValue<'a>,
}
