use scroll::{Pread, Endian};
use crate::dex::error::DexError;
use crate::dex::models::header::RawHeader;
use crate::dex::models::raw::RawProtoId;
use crate::dex::models::proto::Proto;
use crate::dex::constants::sizes::PROTO_ID_ITEM;

pub fn parse(buffer: &[u8], header: &RawHeader, strings: &[String], types: &[String], endian: Endian) -> Result<Vec<Proto>, DexError> {
    let mut protos = Vec::with_capacity(header.proto_ids_size as usize);
    for i in 0..header.proto_ids_size {
        let off = (header.proto_ids_off as usize) + (i as usize * PROTO_ID_ITEM);
        let raw: RawProtoId = buffer.pread_with(off, endian)?;

        let mut parameters = Vec::new();
        if raw.parameters_off != 0 {
            let mut curr = raw.parameters_off as usize;
            let size: u32 = buffer.pread_with(curr, endian)?;
            curr += 4;
            for _ in 0..size {
                let type_idx: u16 = buffer.pread_with(curr, endian)?;
                parameters.push(types.get(type_idx as usize).cloned().unwrap_or_default());
                curr += 2;
            }
        }

        protos.push(Proto {
            shorty: strings.get(raw.shorty_idx as usize).cloned().unwrap_or_default(),
            return_type: types.get(raw.return_type_idx as usize).cloned().unwrap_or_default(),
            parameters,
        });
    }
    Ok(protos)
}
