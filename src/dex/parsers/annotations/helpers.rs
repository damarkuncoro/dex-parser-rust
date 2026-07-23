use crate::dex::error::DexError;
use crate::dex::models::AnnotationItem;
use crate::dex::parsers::traits::DexResolver;
use crate::dex::parsers::encoded_value;
use crate::dex::readers::DexReader;
use scroll::Endian;

pub fn parse_annotation_set<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<AnnotationItem<'a>>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let mut reader = DexReader::new(buffer, endian);
    reader.seek(offset)?;

    let size = reader.read_u32()?;
    let mut items = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let item_off = reader.read_u32()?;
        items.push(parse_annotation_item(buffer, item_off as usize, resolver, endian)?);
    }
    Ok(items)
}

pub fn parse_annotation_item<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationItem<'a>, DexError> {
    let mut reader = DexReader::new(buffer, endian);
    reader.seek(offset)?;

    let visibility = reader.read_u8()?;
    let annotation = encoded_value::parse_annotation(&mut reader, resolver)?;
    Ok(AnnotationItem { visibility, annotation })
}

pub fn parse_annotation_set_ref_list<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<Vec<AnnotationItem<'a>>>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let mut reader = DexReader::new(buffer, endian);
    reader.seek(offset)?;

    let size = reader.read_u32()?;
    let mut per_parameter_annotations = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let set_off = reader.read_u32()?;
        per_parameter_annotations.push(parse_annotation_set(buffer, set_off as usize, resolver, endian)?);
    }
    Ok(per_parameter_annotations)
}
