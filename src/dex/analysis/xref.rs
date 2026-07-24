use crate::dex::models::Dex;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Default)]
pub struct XrefMap {
    pub method_to_methods: HashMap<String, Vec<String>>, // Method A -> [Method B, Method C]
    pub method_to_fields: HashMap<String, Vec<String>>,
    pub method_to_strings: HashMap<String, Vec<String>>,
}

pub struct XrefBuilder;

impl XrefBuilder {
    pub fn build(dex: &Dex) -> XrefMap {
        let mut map = XrefMap::default();

        for class in &dex.class_defs {
            let methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
            for method in methods {
                if let Some(code) = &method.code {
                    let caller_sig = format!("{}->{}", class.name, method.name);

                    for ins in &code.instructions {
                        if let Some(resolved) = &ins.resolved_value {
                            if ins.name.contains("invoke") {
                                map.method_to_methods.entry(caller_sig.clone()).or_default().push(resolved.clone());
                            } else if ins.name.contains("iget") || ins.name.contains("sget") {
                                map.method_to_fields.entry(caller_sig.clone()).or_default().push(resolved.clone());
                            } else if ins.name.contains("const-string") {
                                map.method_to_strings.entry(caller_sig.clone()).or_default().push(resolved.clone());
                            }
                        }
                    }
                }
            }
        }
        map
    }
}
