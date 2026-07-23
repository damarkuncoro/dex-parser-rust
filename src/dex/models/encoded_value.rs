use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EncodedValue {
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
    Array(Vec<EncodedValue>),
    Annotation(EncodedAnnotation),
    Null,
    Boolean(bool),
}

#[derive(Serialize, Clone, Debug)]
pub struct EncodedAnnotation {
    pub type_name: String,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationElement {
    pub name: String,
    pub value: EncodedValue,
}
