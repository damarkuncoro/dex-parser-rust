use serde::Serialize;
use super::Dex;
use crate::dex::analysis::GlobalIntelligence;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Apk<'a> {
    pub dex_files: Vec<Dex<'a>>,
    /// Maps class names to the index of the DEX file that defines them.
    pub class_lookup: HashMap<String, usize>,
    /// APK-wide intelligence findings.
    pub intelligence: GlobalIntelligence,
}

impl<'a> Apk<'a> {
    pub fn new(dex_files: Vec<Dex<'a>>, dex_names: &[String]) -> Self {
        let mut class_lookup = HashMap::new();
        for (i, dex) in dex_files.iter().enumerate() {
            for class in &dex.class_defs {
                class_lookup.insert(class.name.to_string(), i);
            }
        }

        let apk_pre = Self {
            dex_files,
            class_lookup,
            intelligence: GlobalIntelligence::default(),
        };

        // Post-processing to build global intelligence
        let intelligence = GlobalIntelligence::build(&apk_pre, dex_names);

        let mut apk = apk_pre;
        apk.intelligence = intelligence;
        apk
    }
}
