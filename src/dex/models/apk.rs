use serde::Serialize;
use super::Dex;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Apk<'a> {
    pub dex_files: Vec<Dex<'a>>,
    /// Maps class names to the index of the DEX file that defines them.
    pub class_lookup: HashMap<String, usize>,
}

impl<'a> Apk<'a> {
    pub fn new(dex_files: Vec<Dex<'a>>) -> Self {
        let mut class_lookup = HashMap::new();
        for (i, dex) in dex_files.iter().enumerate() {
            for class in &dex.class_defs {
                class_lookup.insert(class.name.to_string(), i);
            }
        }
        Self {
            dex_files,
            class_lookup,
        }
    }
}
