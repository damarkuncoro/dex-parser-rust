use crate::dex::core::models::{DebugInfo};
use crate::dex::parsers::core::traits::DexResolver;
use crate::dex::readers::DexReader;
use crate::trace_parse;

pub mod handlers;

pub struct DebugInfoParser;

impl DebugInfoParser {
    pub fn parse<'a, R: DexResolver<'a>>(
        reader: &mut DexReader<'a>,
        offset: usize,
        resolver: &R,
    ) -> Result<DebugInfo<'a>, crate::dex::error::DexError> {
        parse_debug_info(reader, offset, resolver)
    }
}

pub fn parse_debug_info<'a, R: DexResolver<'a>>(
    reader: &mut DexReader<'a>,
    offset: usize,
    resolver: &R,
) -> Result<DebugInfo<'a>, crate::dex::error::DexError> {
    reader.seek(offset)?;

    trace_parse!("    [DebugInfo] Parsing at offset 0x{:x}", offset);

    let line_start = reader.read_uleb128()? as u32;
    let parameters_size = reader.read_uleb128()? as u32;
    let mut parameters = Vec::with_capacity(parameters_size as usize);

    trace_parse!("      LineStart: {}, Params: {}", line_start, parameters_size);

    for _ in 0..parameters_size {
        let name_idx_plus_1 = reader.read_uleb128()?;
        if name_idx_plus_1 == 0 {
            parameters.push(None);
        } else {
            parameters.push(resolver.resolve_string((name_idx_plus_1 - 1) as u32).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))));
        }
    }

    let entries = handlers::parse_debug_entries(reader, resolver)?;

    Ok(DebugInfo {
        line_start,
        parameters,
        entries,
        _marker: std::marker::PhantomData,
    })
}
