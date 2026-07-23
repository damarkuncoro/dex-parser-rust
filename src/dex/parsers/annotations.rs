use scroll::{Endian, Pread};
use crate::dex::error::DexError;
use crate::dex::models::{AnnotationsDirectory, AnnotationItem, FieldAnnotation, MethodAnnotation, ParameterAnnotation};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::parsers::encoded_value;

pub fn parse<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationsDirectory, DexError> {
    let mut curr = offset;

    let class_annotations_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
    let fields_size: u32 = buffer.pread_with(curr, endian)?; curr += 4;
    let methods_size: u32 = buffer.pread_with(curr, endian)?; curr += 4;
    let parameters_size: u32 = buffer.pread_with(curr, endian)?; curr += 4;

    let class_annotations = if class_annotations_off != 0 {
        parse_annotation_set(buffer, class_annotations_off as usize, resolver, endian)?
    } else {
        Vec::new()
    };

    let mut field_annotations = Vec::with_capacity(fields_size as usize);
    for _ in 0..fields_size {
        let field_idx: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        let annotations_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        field_annotations.push(FieldAnnotation {
            field_idx,
            annotations: parse_annotation_set(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    let mut method_annotations = Vec::with_capacity(methods_size as usize);
    for _ in 0..methods_size {
        let method_idx: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        let annotations_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        method_annotations.push(MethodAnnotation {
            method_idx,
            annotations: parse_annotation_set(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    let mut parameter_annotations = Vec::with_capacity(parameters_size as usize);
    for _ in 0..parameters_size {
        let method_idx: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        let annotations_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        parameter_annotations.push(ParameterAnnotation {
            method_idx,
            annotations: parse_annotation_set_ref_list(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    Ok(AnnotationsDirectory {
        class_annotations,
        field_annotations,
        method_annotations,
        parameter_annotations,
    })
}

fn parse_annotation_set<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<AnnotationItem>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let mut curr = offset;
    let size: u32 = buffer.pread_with(curr, endian)?; curr += 4;
    let mut items = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let item_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        items.push(parse_annotation_item(buffer, item_off as usize, resolver, endian)?);
    }
    Ok(items)
}

fn parse_annotation_item<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationItem, DexError> {
    let mut curr = offset;
    let visibility: u8 = buffer.pread_with(curr, endian)?; curr += 1;
    let annotation = encoded_value::parse_annotation(buffer, &mut curr, resolver, endian)?;
    Ok(AnnotationItem { visibility, annotation })
}

fn parse_annotation_set_ref_list<R: DexResolver>(
    buffer: &[u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<AnnotationItem>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let mut curr = offset;
    let size: u32 = buffer.pread_with(curr, endian)?; curr += 4;
    let mut all_annotations = Vec::new();
    for _ in 0..size {
        let set_off: u32 = buffer.pread_with(curr, endian)?; curr += 4;
        let mut set = parse_annotation_set(buffer, set_off as usize, resolver, endian)?;
        all_annotations.append(&mut set);
    }
    Ok(all_annotations)
}
