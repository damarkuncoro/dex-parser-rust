use crate::dex::models::{Field, Proto, Class};
use crate::dex::models::raw::{RawFieldId, RawProtoId, RawClassDef, RawMethodId};
use crate::dex::utils::access_flags::translate_access_flags;

pub struct DexLinker;

impl DexLinker {
    pub fn link_fields<'a>(raw_fields: &[RawFieldId], strings: &[&'a str], types: &[&'a str]) -> Vec<Field<'a>> {
        raw_fields.iter().map(|raw| {
            Field {
                class: types.get(raw.class_idx as usize).copied().unwrap_or("<invalid>"),
                type_name: types.get(raw.type_idx as usize).copied().unwrap_or("<invalid>"),
                name: strings.get(raw.name_idx as usize).copied().unwrap_or("<invalid>"),
            }
        }).collect()
    }

    pub fn link_protos<'a>(raw_protos: &[RawProtoId], strings: &[&'a str], types: &[&'a str]) -> Vec<Proto<'a>> {
        raw_protos.iter().map(|raw| {
            Proto {
                shorty: strings.get(raw.shorty_idx as usize).copied().unwrap_or("<invalid>"),
                return_type: types.get(raw.return_type_idx as usize).copied().unwrap_or("<invalid>"),
                parameters: Vec::new(),
            }
        }).collect()
    }

    pub fn link_methods(raw_methods: &[RawMethodId], strings: &[&str], types: &[&str], protos: &[Proto]) -> Vec<String> {
        raw_methods.iter().map(|raw| {
            let class_name = types.get(raw.class_idx as usize).copied().unwrap_or("<invalid>");
            let method_name = strings.get(raw.name_idx as usize).copied().unwrap_or("<invalid>");
            let proto = protos.get(raw.proto_idx as usize);

            if let Some(p) = proto {
                format!("{}->{}({}){}", class_name, method_name, p.parameters.join(""), p.return_type)
            } else {
                format!("{}->{}", class_name, method_name)
            }
        }).collect()
    }

    pub fn link_class<'a>(
        raw_def: &RawClassDef,
        strings: &[&'a str],
        types: &[&'a str],
    ) -> Class<'a> {
        let name = types.get(raw_def.class_idx as usize).copied().unwrap_or("<invalid>");
        let superclass = types.get(raw_def.superclass_idx as usize).copied().unwrap_or("None");

        let source_file = if raw_def.source_file_idx == 0xffffffff {
            None
        } else {
            strings.get(raw_def.source_file_idx as usize).copied()
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
