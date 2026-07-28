use crate::dex::core::models::{Class, DexMetadata};
use crate::dex::readers::DexReader;
use crate::dex::error::DexError;
use crate::dex::core::linker::DexLinker;
use crate::dex::parsers::identifiers::strings::StringSection;
use crate::dex::parsers::definitions::classes;
use crate::dex::parsers::core::traits::SimpleResolver;
use crate::dex::parsers::pipeline::stage_atomic::AtomicResults;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use std::sync::Arc;
use parking_lot::Mutex;
use scroll::Endian;

pub struct LinkingResults<'a> {
    pub metadata: DexMetadata<'a>,
    pub classes: Vec<Class<'a>>,
}

pub fn run<'a>(
    buffer: &'a [u8],
    reader: &mut DexReader<'a>,
    atomic: &AtomicResults,
    tracker: Arc<Mutex<ByteTracker>>,
    endian: Endian,
) -> Result<LinkingResults<'a>, DexError> {
    let strings = StringSection::resolve_strings(reader, &atomic.raw_data.string_offsets)?;

    let resolved_types: Vec<&'a [u8]> = atomic.raw_data.type_indices.iter()
        .map(|&idx| strings.get(idx as usize).copied().unwrap_or(b"<invalid>"))
        .collect();

    let protos = DexLinker::link_protos(buffer, &atomic.raw_data.raw_protos, &strings, &resolved_types, endian, tracker.clone())?;
    let fields = DexLinker::link_fields(&atomic.raw_data.raw_fields, &strings, &resolved_types);
    let methods_display = DexLinker::link_methods(&atomic.raw_data.raw_methods, &strings, &resolved_types, &protos);

    let resolver = SimpleResolver {
        strings: strings.clone(),
        types: resolved_types.clone(),
        methods: methods_display.clone(),
        fields: fields.clone(),
    };

    let classes = classes::linker::parse_linked(
        buffer,
        &atomic.header,
        &strings,
        &resolved_types,
        &fields,
        &methods_display,
        &atomic.raw_data.raw_classes,
        &atomic.raw_data.raw_methods,
        &resolver,
        endian,
        tracker
    )?;

    Ok(LinkingResults {
        metadata: DexMetadata {
            strings,
            types: resolved_types,
            protos,
            fields,
            methods: methods_display,
        },
        classes,
    })
}
