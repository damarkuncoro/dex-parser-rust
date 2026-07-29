use scroll::{Pread, LE};
use crate::dex::error::DexError;
use crate::dex::core::models::{Manifest, Component};

pub struct ElementProcessor<'a> {
    buffer: &'a [u8],
}

impl<'a> ElementProcessor<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    pub fn process(&self, offset: usize, string_pool: &[String], manifest: &mut Manifest, stack: &[String]) -> Result<(), DexError> {
        let current_el = stack.last().map(|s| s.as_str()).unwrap_or("");

        let header_size: u16 = self.buffer.pread_with(offset + 2, LE).unwrap_or(0);
        let attr_start_off: u16 = self.buffer.pread_with(offset + 24, LE).unwrap_or(0);
        let attr_count: u16 = self.buffer.pread_with(offset + 28, LE).unwrap_or(0);

        let attr_start = offset + header_size as usize + attr_start_off as usize;

        let mut attrs = std::collections::HashMap::new();
        for i in 0..attr_count {
            let attr_offset = attr_start + (i as usize * 20);
            let attr_name_idx: u32 = self.buffer.pread_with(attr_offset + 4, LE).unwrap_or(0);
            let attr_val_idx: i32 = self.buffer.pread_with(attr_offset + 8, LE).unwrap_or(-1);

            let attr_name = string_pool.get(attr_name_idx as usize).map(|s| s.as_str()).unwrap_or("");
            let attr_value = if attr_val_idx >= 0 {
                string_pool.get(attr_val_idx as usize).cloned().unwrap_or_default()
            } else {
                let data: u32 = self.buffer.pread_with(attr_offset + 16, LE).unwrap_or(0);
                format!("0x{:08x}", data)
            };
            attrs.insert(attr_name.to_string(), attr_value);
        }

        let name_val = attrs.iter()
            .find(|(k, _)| k.as_str() == "name" || k.ends_with(":name"))
            .map(|(_, v)| v.clone())
            .unwrap_or_default();

        match current_el {
            e if e == "manifest" || e.ends_with(":manifest") => {
                if let Some(p) = attrs.get("package") {
                    manifest.package_name = p.clone();
                } else if let Some(p) = attrs.iter().find(|(k, _)| k.ends_with(":package")).map(|(_, v)| v) {
                    manifest.package_name = p.clone();
                }

                if manifest.package_name.is_empty() {
                    for val in attrs.values() {
                        if val.contains('.') && val.split('.').count() >= 2 && !val.contains(' ') && !val.starts_with("0x") {
                             manifest.package_name = val.clone();
                             break;
                        }
                    }
                }
            }
            e if e == "uses-permission" || e.ends_with(":uses-permission") => {
                if !name_val.is_empty() {
                    manifest.permissions.push(name_val);
                } else {
                    if let Some(p) = attrs.values().find(|v| v.contains("android.permission")) {
                        manifest.permissions.push(p.clone());
                    }
                }
            }
            e if e == "activity" || e == "service" || e == "receiver" || e == "provider" ||
                 e.ends_with(":activity") || e.ends_with(":service") || e.ends_with(":receiver") || e.ends_with(":provider") => {

                let base_name = e.split(':').last().unwrap_or(e);
                let comp = Component { name: name_val, ..Default::default() };
                match base_name {
                    "activity" => manifest.activities.push(comp),
                    "service" => manifest.services.push(comp),
                    "receiver" => manifest.receivers.push(comp),
                    "provider" => manifest.providers.push(comp),
                    _ => {}
                }
            }
            e if e == "action" || e.ends_with(":action") => {
                if stack.len() >= 3 {
                    let parent = &stack[stack.len()-3];
                    let parent_base = parent.split(':').last().unwrap_or(parent);
                    self.add_action_to_last_component(manifest, parent_base, &name_val);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn add_action_to_last_component(&self, manifest: &mut Manifest, parent: &str, action: &str) {
        let list = match parent {
            "activity" => &mut manifest.activities,
            "service" => &mut manifest.services,
            "receiver" => &mut manifest.receivers,
            _ => return,
        };
        if let Some(comp) = list.last_mut() {
            if comp.intent_filters.is_empty() { comp.intent_filters.push(Default::default()); }
            comp.intent_filters.last_mut().unwrap().actions.push(action.to_string());
        }
    }
}
