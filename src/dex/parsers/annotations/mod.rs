pub mod helpers;

use crate::dex::error::DexError;
use crate::dex::models::{AnnotationsDirectory, FieldAnnotation, MethodAnnotation, ParameterAnnotation};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use scroll::Endian;
use self::helpers::{parse_annotation_set, parse_annotation_set_ref_list};

pub fn parse<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationsDirectory<'a>, DexError> {
    let mut reader = DexReader::new(buffer, endian);
    reader.seek(offset)?;

    let class_annotations_off = reader.read_u32()?;
    let fields_size = reader.read_u32()?;
    let methods_size = reader.read_u32()?;
    let parameters_size = reader.read_u32()?;

    let class_annotations = if class_annotations_off != 0 {
        parse_annotation_set(buffer, class_annotations_off as usize, resolver, endian)?
    } else {
        Vec::new()
    };

    let mut field_annotations = Vec::with_capacity(fields_size as usize);
    for _ in 0..fields_size {
        let field_idx = reader.read_u32()?;
        let annotations_off = reader.read_u32()?;
        field_annotations.push(FieldAnnotation {
            field_idx,
            annotations: parse_annotation_set(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    let mut method_annotations = Vec::with_capacity(methods_size as usize);
    for _ in 0..methods_size {
        let method_idx = reader.read_u32()?;
        let annotations_off = reader.read_u32()?;
        method_annotations.push(MethodAnnotation {
            method_idx,
            annotations: parse_annotation_set(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    let mut parameter_annotations = Vec::with_capacity(parameters_size as usize);
    for _ in 0..parameters_size {
        let method_idx = reader.read_u32()?;
        let annotations_off = reader.read_u32()?;
        parameter_annotations.push(ParameterAnnotation {
            method_idx,
            annotations_per_parameter: parse_annotation_set_ref_list(buffer, annotations_off as usize, resolver, endian)?,
        });
    }

    Ok(AnnotationsDirectory {
        class_annotations,
        field_annotations,
        method_annotations,
        parameter_annotations,
    })
}
