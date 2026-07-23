use serde::Serialize;
use super::encoded_value::EncodedAnnotation;

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationsDirectory {
    pub class_annotations: Vec<AnnotationItem>,
    pub field_annotations: Vec<FieldAnnotation>,
    pub method_annotations: Vec<MethodAnnotation>,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationItem {
    pub visibility: u8,
    pub annotation: EncodedAnnotation,
}

#[derive(Serialize, Clone, Debug)]
pub struct FieldAnnotation {
    pub field_idx: u32,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MethodAnnotation {
    pub method_idx: u32,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ParameterAnnotation {
    pub method_idx: u32,
    pub annotations: Vec<AnnotationItem>, // This is actually complex in DEX, but simplified for now
}
