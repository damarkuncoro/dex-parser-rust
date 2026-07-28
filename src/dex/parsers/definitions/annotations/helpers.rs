use crate::dex::core::models::AnnotationItem;
use crate::dex::error::DexError;
use crate::dex::parsers::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::dex::parsers::definitions::encoded_value;
use scroll::{Endian};

pub fn parse_annotation_set<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<AnnotationItem<'a>>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let mut reader = DexReader::new(buffer, endian);
    parse_annotation_set_with_reader(&mut reader, offset, resolver)
}

pub fn parse_annotation_set_with_reader<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<Vec<AnnotationItem<'a>>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let saved_pos = reader.position();
    reader.seek(offset)?;

    let size = reader.read_u32()?;
    let mut items = Vec::with_capacity(size as usize);
    let mut offsets = Vec::with_capacity(size as usize);
    for _ in 0..size {
        offsets.push(reader.read_u32()?);
    }

    for off in offsets {
        items.push(parse_annotation_item_with_reader(reader, off as usize, resolver)?);
    }

    reader.seek(saved_pos).ok(); // try to restore position
    Ok(items)
}

pub fn parse_annotation_set_ref_list<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<Vec<Vec<AnnotationItem<'a>>>, DexError> {
    let mut reader = DexReader::new(buffer, endian);
    parse_annotation_set_ref_list_with_reader(&mut reader, offset, resolver)
}

pub fn parse_annotation_set_ref_list_with_reader<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<Vec<Vec<AnnotationItem<'a>>>, DexError> {
    if offset == 0 { return Ok(Vec::new()); }
    let saved_pos = reader.position();
    reader.seek(offset)?;

    let size = reader.read_u32()?;
    let mut result = Vec::with_capacity(size as usize);
    let mut offsets = Vec::with_capacity(size as usize);
    for _ in 0..size {
        offsets.push(reader.read_u32()?);
    }

    for off in offsets {
        result.push(parse_annotation_set_with_reader(reader, off as usize, resolver)?);
    }

    reader.seek(saved_pos).ok();
    Ok(result)
}

pub fn parse_annotation_item<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    offset: usize,
    resolver: &R,
    endian: Endian,
) -> Result<AnnotationItem<'a>, DexError> {
    let mut reader = DexReader::new(buffer, endian);
    parse_annotation_item_with_reader(&mut reader, offset, resolver)
}

pub fn parse_annotation_item_with_reader<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<AnnotationItem<'a>, DexError> {
    let saved_pos = reader.position();
    reader.seek(offset)?;

    let visibility = reader.read_u8()?;
    let annotation = encoded_value::parse_encoded_annotation(reader, resolver)?;

    reader.seek(saved_pos).ok();
    Ok(AnnotationItem { visibility, annotation })
}
