pub mod strings;
pub mod types;
pub mod protos;
pub mod fields;
pub mod methods;
pub mod classes;
pub mod class_data;
pub mod code;
pub mod traits;

use scroll::{Pread, Endian};
use crate::dex::models::{Dex, header::RawHeader};
use crate::dex::error::DexError;
use crate::dex::context::DexContext;
use crate::dex::constants::{ENDIAN_CONSTANT, offsets::ENDIAN_TAG};
use crate::dex::utils::calculate_adler32;

pub struct DexParser<'a> {
    buffer: &'a [u8],
}

impl<'a> DexParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn parse(self) -> Result<Dex, DexError> {
        let (header, endian) = self.parse_header()?;
        let mut ctx = DexContext::new(self.buffer, header, endian);

        self.parse_metadata(&mut ctx)?;
        let classes = classes::parse(ctx.buffer, &ctx.header, &ctx.protos, &ctx, ctx.endian)?;

        Ok(Dex {
            header: ctx.header,
            strings: ctx.strings,
            types: ctx.types,
            protos: ctx.protos,
            fields: ctx.fields,
            methods: ctx.methods,
            classes,
        })
    }

    fn parse_header(&self) -> Result<(RawHeader, Endian), DexError> {
        let endian_tag: u32 = self.buffer.pread_with(ENDIAN_TAG, Endian::Little)?;
        let endian = if endian_tag == ENDIAN_CONSTANT { Endian::Little } else { Endian::Big };
        let header: RawHeader = self.buffer.pread_with(0, endian)?;

        let header_checksum: u32 = self.buffer.pread_with(8, Endian::Little)?;
        if header_checksum != calculate_adler32(&self.buffer[12..]) {
            // Optional: Log warning or handle checksum mismatch
        }

        Ok((header, endian))
    }

    fn parse_metadata(&self, ctx: &mut DexContext) -> Result<(), DexError> {
        ctx.strings = strings::parse(ctx.buffer, ctx.header.string_ids_size, ctx.header.string_ids_off, ctx.endian)?;
        ctx.types = types::parse(ctx.buffer, &ctx.header, &ctx.strings, ctx.endian)?;
        ctx.protos = protos::parse(ctx.buffer, &ctx.header, &ctx.strings, &ctx.types, ctx.endian)?;
        ctx.fields = fields::parse(ctx.buffer, &ctx.header, &ctx.strings, &ctx.types, ctx.endian)?;
        ctx.methods = methods::parse(ctx.buffer, &ctx.header, &ctx.strings, &ctx.types, ctx.endian)?;
        Ok(())
    }
}
