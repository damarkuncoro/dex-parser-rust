use crate::dex::core::models::header::RawHeader;
use crate::dex::core::models::map_list::MapList;
use crate::dex::readers::DexReader;
use crate::dex::error::DexError;
use crate::dex::validator::DexValidator;
use crate::dex::parsers::core::{HeaderParser, MapProcessor};
use crate::dex::parsers::core::map_processor::MapProcessorResults;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct AtomicResults {
    pub header: RawHeader,
    pub map_list: MapList,
    pub raw_data: MapProcessorResults,
}

pub fn run(reader: &mut DexReader, buffer: &[u8], tracker: Arc<Mutex<ByteTracker>>) -> Result<AtomicResults, DexError> {
    let header_type = HeaderParser::parse(reader)?;
    let common_header = header_type.common();

    DexValidator::new().validate(buffer, &common_header)?;
    let map_list = crate::dex::parsers::core::parse_map_list(buffer, common_header.map_off as usize, reader.endian())?;

    {
        let mut t = tracker.lock();
        t.mark(0, 112);
        t.mark(common_header.map_off as usize, map_list.items.len() * 12 + 4);
    }

    let raw_data = MapProcessor::process(reader, &map_list)?;

    Ok(AtomicResults {
        header: common_header,
        map_list,
        raw_data,
    })
}
