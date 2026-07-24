use crate::dex::error::DexError;
use crate::dex::core::models::{EncodedMethod};
use crate::dex::core::models::raw::RawMethodId;
use crate::dex::parsers::{class_data, code, traits::DexResolver};
use crate::dex::core::utils::access_flags::translate_access_flags;
use crate::trace_parse;

pub fn link_methods<'a, R: DexResolver<'a> + Sync + Send>(
    reader: &mut crate::dex::readers::DexReader<'a>,
    class_data_methods: &[class_data::RawEncodedMethod],
    methods_display: &[String],
    raw_methods: &[RawMethodId],
    resolver: &R,
) -> Result<Vec<EncodedMethod<'a>>, DexError> {
    let mut result = Vec::with_capacity(class_data_methods.len());

    for m in class_data_methods {
        let signature = &methods_display[m.method_idx as usize];
        let raw = &raw_methods[m.method_idx as usize];
        let name = resolver.resolve_string(raw.name_idx).map(|b| format!("{}", crate::dex::core::utils::mutf8::Mutf8Display(b))).unwrap_or_else(|| "unknown".to_string());

        trace_parse!("  [MethodLinker] Linking: {}, CodeOff: 0x{:x}", signature, m.code_off);

        let code = if m.code_off != 0 {
            Some(code::parse_code_item(reader, m.code_off as usize, resolver)?)
        } else {
            None
        };

        result.push(EncodedMethod {
            name,
            signature: signature.clone(),
            access_flags: m.access_flags,
            access_flags_text: translate_access_flags(m.access_flags, true),
            code_off: m.code_off,
            code,
        });
    }

    Ok(result)
}
