use crate::dex::core::models::{EncodedField, Field};
use crate::dex::parsers::class_data;
use crate::dex::core::utils::access_flags::translate_access_flags;
use crate::dex::parsers::traits::StringResolver;

pub fn link_fields<'a, R: StringResolver<'a>>(
    class_data_fields: &[class_data::RawEncodedField],
    fields: &[Field<'a>],
    _resolver: &R,
) -> Vec<EncodedField<'a>> {
    class_data_fields.iter().map(|f| {
        let field = &fields[f.field_idx as usize];
        EncodedField {
            name: field.name.clone(),
            type_name: field.type_name.clone(),
            access_flags: f.access_flags,
            access_flags_text: translate_access_flags(f.access_flags, false),
            _marker: std::marker::PhantomData,
        }
    }).collect()
}
