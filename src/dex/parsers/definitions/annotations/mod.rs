use crate::dex::core::models::{AnnotationsDirectory, FieldAnnotation, MethodAnnotation, ParameterAnnotation};
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use scroll::Endian;

pub mod helpers;

pub struct AnnotationParser;

impl AnnotationParser {
    pub fn parse<'a, R: DexResolver<'a>>(
        reader: &mut DexReader<'a>,
        offset: usize,
        resolver: &R,
    ) -> Result<AnnotationsDirectory<'a>, crate::dex::error::DexError> {
        parse_annotations_directory_with_reader(reader, offset, resolver)
    }
}

pub fn parse_annotations_directory<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationsDirectory<'a>, crate::dex::error::DexError> {
    let mut reader = DexReader::new(buffer, endian);
    parse_annotations_directory_with_reader(&mut reader, offset, resolver)
}

pub fn parse_annotations_directory_with_reader<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<AnnotationsDirectory<'a>, crate::dex::error::DexError> {
    reader.seek(offset)?;
    let _buffer = reader.buffer();
    let _endian = reader.endian();

    let class_off = reader.read_u32()?;
    let fields_size = reader.read_u32()?;
    let methods_size = reader.read_u32()?;
    let params_size = reader.read_u32()?;

    let class_annotations = if class_off != 0 {
        helpers::parse_annotation_set_with_reader(reader, class_off as usize, resolver)?
    } else {
        Vec::new()
    };

    let mut field_annotations = Vec::with_capacity(fields_size as usize);
    for _ in 0..fields_size {
        let field_idx = reader.read_u32()?;
        let off = reader.read_u32()?;
        field_annotations.push(FieldAnnotation {
            field_idx,
            annotations: helpers::parse_annotation_set_with_reader(reader, off as usize, resolver)?,
        });
    }

    let mut method_annotations = Vec::with_capacity(methods_size as usize);
    for _ in 0..methods_size {
        let method_idx = reader.read_u32()?;
        let off = reader.read_u32()?;
        method_annotations.push(MethodAnnotation {
            method_idx,
            annotations: helpers::parse_annotation_set_with_reader(reader, off as usize, resolver)?,
        });
    }

    let mut parameter_annotations = Vec::with_capacity(params_size as usize);
    for _ in 0..params_size {
        let method_idx = reader.read_u32()?;
        let off = reader.read_u32()?;
        parameter_annotations.push(ParameterAnnotation {
            method_idx,
            annotations_per_parameter: helpers::parse_annotation_set_ref_list_with_reader(reader, off as usize, resolver)?,
        });
    }

    Ok(AnnotationsDirectory {
        class_annotations,
        field_annotations,
        method_annotations,
        parameter_annotations,
        _marker: std::marker::PhantomData,
    })
}
