use crate::dex::error::DexError;
use crate::dex::models::{EncodedMethod};
use crate::dex::models::raw::RawMethodId;
use crate::dex::parsers::{class_data, code, traits::DexResolver};
use crate::dex::utils::access_flags::translate_access_flags;
use scroll::Endian;

pub fn link_methods<'a, R: DexResolver<'a>>(
    buffer: &'a [u8],
    raw_methods: &[class_data::RawEncodedMethod],
    raw_method_ids: &[RawMethodId],
    strings: &[&'a str],
    methods_sigs: &[String],
    resolver: &R,
    endian: Endian,
) -> Result<Vec<EncodedMethod<'a>>, DexError> {
    let mut last_idx = 0u64;
    let mut methods = Vec::with_capacity(raw_methods.len());
    for raw in raw_methods {
        let method_idx = (last_idx + raw.method_idx_diff) as usize;
        last_idx = method_idx as u64;

        let raw_mid = raw_method_ids.get(method_idx).ok_or_else(|| {
            DexError::InvalidIndex(format!("Method index {}", method_idx))
        })?;
        let name = strings.get(raw_mid.name_idx as usize).copied().unwrap_or("<invalid>");
        let signature = methods_sigs.get(method_idx).cloned().unwrap_or_default();

        let code = if raw.code_off != 0 {
            Some(code::parse(buffer, raw.code_off as usize, resolver, endian)?)
        } else {
            None
        };

        methods.push(EncodedMethod {
            name,
            signature,
            access_flags: raw.access_flags as u32,
            access_flags_text: translate_access_flags(raw.access_flags as u32, true),
            code_off: raw.code_off,
            code,
        });
    }
    Ok(methods)
}
