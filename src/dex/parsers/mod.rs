pub mod header;
pub mod strings;
pub mod types;
pub mod protos;
pub mod fields;
pub mod methods;
pub mod classes;
pub mod class_data;
pub mod code;
pub mod traits;
pub mod encoded_value;
pub mod annotations;
pub mod debug_info;
pub mod map_list;
pub mod map_processor;
pub mod method_handles;
pub mod call_sites;

use crate::dex::core::constants::{offsets::ENDIAN_TAG, ENDIAN_CONSTANT};
use crate::dex::error::DexError;
use crate::dex::core::models::{Dex, DexMetadata};
use crate::dex::readers::DexReader;
use crate::dex::validator::DexValidator;
use crate::dex::core::linker::DexLinker;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use crate::dex::analysis::{EntropyAnalyzer, StringScanner};
use self::header::{HeaderParser};
use self::strings::StringSection;
use self::traits::SimpleResolver;
use scroll::Endian;
use std::sync::{Arc};
use parking_lot::Mutex;

pub struct DexParser<'a> {
    buffer: &'a [u8],
}

impl<'a> DexParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn parse(buffer: &'a [u8]) -> Result<Dex<'a>, DexError> {
        Self::new(buffer).parse_internal()
    }

    fn parse_internal(self) -> Result<Dex<'a>, DexError> {
        let tracker = Arc::new(Mutex::new(ByteTracker::new(self.buffer.len())));
        let endian = self.detect_endian()?;
        let mut reader = DexReader::new(self.buffer, endian).with_tracker(tracker.clone());

        let header_type = HeaderParser::parse(&mut reader)?;
        let common_header = header_type.common();

        DexValidator::new().validate(self.buffer, &common_header)?;
        let map_list = map_list::parse(self.buffer, common_header.map_off as usize, endian)?;
        {
            let mut t = tracker.lock();
            t.mark(0, 112);
            t.mark(common_header.map_off as usize, map_list.items.len() * 12 + 4);
        }

        let map_results = map_processor::MapProcessor::process(&mut reader, &map_list)?;

        let strings = StringSection::resolve_strings(&mut reader, &map_results.string_offsets)?;

        let resolved_types: Vec<&'a [u8]> = map_results.type_indices.iter()
            .map(|&idx| strings.get(idx as usize).copied().unwrap_or(b"<invalid>"))
            .collect();

        let protos = DexLinker::link_protos(self.buffer, &map_results.raw_protos, &strings, &resolved_types, endian, tracker.clone())?;

        let fields = DexLinker::link_fields(&map_results.raw_fields, &strings, &resolved_types);
        let methods_display = DexLinker::link_methods(&map_results.raw_methods, &strings, &resolved_types, &protos);

        let resolver = SimpleResolver {
            strings: strings.clone(),
            types: resolved_types.clone(),
            methods: methods_display.clone(),
            fields: fields.clone(),
        };

        let classes = classes::linker::parse_linked(
            self.buffer,
            &common_header,
            &strings,
            &resolved_types,
            &fields,
            &methods_display,
            &map_results.raw_classes,
            &map_results.raw_methods,
            &resolver,
            endian,
            tracker.clone()
        )?;

        let byte_gaps = tracker.lock().get_gaps();
        let gap_analysis = EntropyAnalyzer::analyze_gaps(self.buffer, &byte_gaps);
        let scan_results = StringScanner::scan(&strings);
        let analysis_report = crate::dex::core::models::analysis::AnalysisReport::new(gap_analysis, scan_results);

        Ok(Dex {
            header: common_header.clone(),
            metadata: DexMetadata {
                strings,
                types: resolved_types,
                protos,
                fields,
                methods: methods_display,
            },
            class_defs: classes,
            map_list,
            method_handles: map_results.method_handles,
            call_sites: map_results.call_sites,
            byte_gaps,
            analysis: analysis_report,
        })
    }

    fn detect_endian(&self) -> Result<Endian, DexError> {
        use scroll::Pread;
        let tag: u32 = self.buffer.pread_with(ENDIAN_TAG, Endian::Little).map_err(DexError::ScrollError)?;
        if tag == ENDIAN_CONSTANT { Ok(Endian::Little) } else { Ok(Endian::Big) }
    }
}

impl DexParser<'static> {
    pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<Dex<'static>, DexError> {
        let mut file = std::fs::File::open(path).map_err(DexError::IoError)?;
        let mut buffer = Vec::new();
        use std::io::Read;
        file.read_to_end(&mut buffer).map_err(DexError::IoError)?;
        let leaked: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        DexParser::parse(leaked)
    }
}
