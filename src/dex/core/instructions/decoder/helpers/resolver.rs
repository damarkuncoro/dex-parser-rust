use crate::dex::core::instructions::opcodes::IndexType;
use crate::dex::parsers::core::traits::DexResolver;
use crate::dex::core::utils::mutf8::Mutf8Display;

pub fn resolve_xref<'a, R: DexResolver<'a>>(
    description: &mut String,
    index_type: IndexType,
    index: u32,
    resolver: &R,
) -> String {
    let resolved = match index_type {
        IndexType::String => resolver.resolve_string(index)
            .map(|b| format!("\"{}\"", Mutf8Display(b)))
            .unwrap_or_else(|| format!("string@{:04x}", index)),
        IndexType::Type => resolver.resolve_type(index)
            .map(|b| format!("{}", Mutf8Display(b)))
            .unwrap_or_else(|| format!("type@{:04x}", index)),
        IndexType::Method => resolver.resolve_method(index)
            .unwrap_or_else(|| format!("meth@{:04x}", index)),
        IndexType::Field => resolver.resolve_field(index)
            .map(|f| format!("{}->{}:{}", f.class, f.name, f.type_name))
            .unwrap_or_else(|| format!("field@{:04x}", index)),
        IndexType::CallSite => format!("call_site@{:04x}", index),
        IndexType::MethodHandle => format!("method_handle@{:04x}", index),
        IndexType::Proto => format!("proto@{:04x}", index),
        IndexType::None => String::new(),
    };

    if description.contains("string@") { *description = description.replace("string@", &format!("{}", resolved)); }
    else if description.contains("type@") { *description = description.replace("type@", &format!("{}", resolved)); }
    else if description.contains("meth@") { *description = description.replace("meth@", &format!("{}", resolved)); }
    else if description.contains("field@") { *description = description.replace("field@", &format!("{}", resolved)); }
    else if description.contains("call_site@") { *description = description.replace("call_site@", &format!("{}", resolved)); }
    else if description.contains("method_handle@") { *description = description.replace("method_handle@", &format!("{}", resolved)); }
    else if description.contains("proto@") { *description = description.replace("proto@", &format!("{}", resolved)); }
    else if index_type != IndexType::None { *description = format!("{} {}", description, resolved); }

    resolved
}
