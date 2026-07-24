use crate::dex::core::models::{Field, Proto, Class};
use crate::dex::core::models::raw::{RawFieldId, RawProtoId, RawClassDef, RawMethodId};
use crate::dex::core::utils::access_flags::translate_access_flags;
use crate::dex::core::utils::mutf8::Mutf8Display;

pub struct DexLinker;

impl DexLinker {
    pub fn link_fields<'a>(raw_fields: &[RawFieldId], strings: &[&'a [u8]], types: &[&'a [u8]]) -> Vec<Field<'a>> {
        raw_fields.iter().map(|raw| {
            let class_bytes = types.get(raw.class_idx as usize).copied().unwrap_or(b"<invalid>");
            let type_bytes = types.get(raw.type_idx as usize).copied().unwrap_or(b"<invalid>");
            let name_bytes = strings.get(raw.name_idx as usize).copied().unwrap_or(b"<invalid>");

            Field {
                class: format!("{}", Mutf8Display(class_bytes)),
                type_name: format!("{}", Mutf8Display(type_bytes)),
                name: format!("{}", Mutf8Display(name_bytes)),
                _marker: std::marker::PhantomData,
            }
        }).collect()
    }

    pub fn link_protos<'a>(raw_protos: &[RawProtoId], strings: &[&'a [u8]], types: &[&'a [u8]]) -> Vec<Proto<'a>> {
        raw_protos.iter().map(|raw| {
            let shorty_bytes = strings.get(raw.shorty_idx as usize).copied().unwrap_or(b"<invalid>");
            let return_bytes = types.get(raw.return_type_idx as usize).copied().unwrap_or(b"<invalid>");

            Proto {
                shorty: format!("{}", Mutf8Display(shorty_bytes)),
                return_type: format!("{}", Mutf8Display(return_bytes)),
                parameters: Vec::new(),
                _marker: std::marker::PhantomData,
            }
        }).collect()
    }

    pub fn link_methods(raw_methods: &[RawMethodId], strings: &[&[u8]], types: &[&[u8]], protos: &[Proto]) -> Vec<String> {
        raw_methods.iter().map(|raw| {
            let class_bytes = types.get(raw.class_idx as usize).copied().unwrap_or(b"<invalid>");
            let method_bytes = strings.get(raw.name_idx as usize).copied().unwrap_or(b"<invalid>");
            let proto = protos.get(raw.proto_idx as usize);

            let class_name = format!("{}", Mutf8Display(class_bytes));
            let method_name = format!("{}", Mutf8Display(method_bytes));

            if let Some(p) = proto {
                format!("{}->{}({}){}", class_name, method_name, p.parameters.join(""), p.return_type)
            } else {
                format!("{}->{}", class_name, method_name)
            }
        }).collect()
    }

    pub fn link_class<'a>(
        raw_def: &RawClassDef,
        strings: &[&'a [u8]],
        types: &[&'a [u8]],
    ) -> Class<'a> {
        let name_bytes = types.get(raw_def.class_idx as usize).copied().unwrap_or(b"<invalid>");
        let super_bytes = types.get(raw_def.superclass_idx as usize).copied().unwrap_or(b"None");

        let name = format!("{}", Mutf8Display(name_bytes));
        let superclass = format!("{}", Mutf8Display(super_bytes));

        let source_file = if raw_def.source_file_idx == 0xffffffff {
            None
        } else {
            strings.get(raw_def.source_file_idx as usize).map(|&b| format!("{}", Mutf8Display(b)))
        };

        Class {
            class_idx: raw_def.class_idx,
            name,
            access_flags: raw_def.access_flags,
            access_flags_text: translate_access_flags(raw_def.access_flags, false),
            superclass,
            interfaces: Vec::new(),
            source_file_idx: raw_def.source_file_idx as i32,
            source_file,
            static_fields: Vec::new(),
            instance_fields: Vec::new(),
            direct_methods: Vec::new(),
            virtual_methods: Vec::new(),
            annotations: None,
            static_values: Vec::new(),
        }
    }
}
