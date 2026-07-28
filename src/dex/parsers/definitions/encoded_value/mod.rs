pub mod utils;
pub mod parser;

pub struct EncodedValueParser;

impl EncodedValueParser {
    pub fn parse_array<'a, R: crate::dex::parsers::core::traits::DexResolver<'a>>(
        reader: &mut crate::dex::readers::DexReader<'a>,
        resolver: &R,
    ) -> Result<Vec<crate::dex::core::models::encoded_value::EncodedValue<'a>>, crate::dex::error::DexError> {
        parser::parse_encoded_array(reader, resolver)
    }
}

pub use parser::{parse_encoded_value, parse_encoded_annotation, parse_encoded_array};
