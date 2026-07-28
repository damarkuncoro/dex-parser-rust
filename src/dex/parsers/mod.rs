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
pub mod pipeline;

use crate::dex::core::constants::{offsets::ENDIAN_TAG, ENDIAN_CONSTANT};
use crate::dex::error::DexError;
use crate::dex::core::models::{Dex};
use crate::dex::readers::DexReader;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use crate::analysis::core::config::CompiledConfig;
use scroll::Endian;
use std::sync::{Arc};
use parking_lot::Mutex;

pub struct DexParser<'a> {
    buffer: &'a [u8],
    config: Arc<CompiledConfig>,
}

impl<'a> DexParser<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            config: CompiledConfig::compile(Default::default()).unwrap()
        }
    }

    pub fn with_config(mut self, config: Arc<CompiledConfig>) -> Self {
        self.config = config;
        self
    }

    pub fn parse(buffer: &'a [u8]) -> Result<Dex<'a>, DexError> {
        Self::new(buffer).parse_internal()
    }

    pub fn parse_with_config(buffer: &'a [u8], config: Arc<CompiledConfig>) -> Result<Dex<'a>, DexError> {
        Self::new(buffer).with_config(config).parse_internal()
    }

    pub fn parse_internal(self) -> Result<Dex<'a>, DexError> {
        let tracker = Arc::new(Mutex::new(ByteTracker::new(self.buffer.len())));
        let endian = self.detect_endian()?;
        let mut reader = DexReader::new(self.buffer, endian).with_tracker(tracker.clone());

        // Stage 1: Atomic Data Extraction
        let atomic = pipeline::stage_atomic::run(&mut reader, self.buffer, tracker.clone())?;

        // Stage 2: Logical Linking
        let linking = pipeline::stage_linking::run(self.buffer, &mut reader, &atomic, tracker.clone(), endian)?;

        // Stage 3: Post-Parsing Analysis
        let analysis = pipeline::stage_analysis::run(self.buffer, &linking.metadata.strings, &linking.classes, tracker, self.config.clone());

        Ok(Dex {
            header: atomic.header,
            metadata: linking.metadata,
            class_defs: linking.classes,
            map_list: atomic.map_list,
            method_handles: atomic.raw_data.method_handles,
            call_sites: atomic.raw_data.call_sites,
            byte_gaps: analysis.byte_gaps,
            analysis: analysis.report,
            analysis_config: self.config.config.clone(),
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
