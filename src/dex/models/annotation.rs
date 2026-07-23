use serde::Serialize;
use super::encoded_value::EncodedAnnotation;

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationsDirectory<'a> {
    pub class_annotations: Vec<AnnotationItem<'a>>,
    pub field_annotations: Vec<FieldAnnotation<'a>>,
    pub method_annotations: Vec<MethodAnnotation<'a>>,
    pub parameter_annotations: Vec<ParameterAnnotation<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AnnotationItem<'a> {
    pub visibility: u8,
    pub annotation: EncodedAnnotation<'a>,
}

#[derive(Serialize, Clone, Debug)]
pub struct FieldAnnotation<'a> {
    pub field_idx: u32,
    pub annotations: Vec<AnnotationItem<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct MethodAnnotation<'a> {
    pub method_idx: u32,
    pub annotations: Vec<AnnotationItem<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ParameterAnnotation<'a> {
    pub method_idx: u32,
    pub annotations_per_parameter: Vec<Vec<AnnotationItem<'a>>>,
}
