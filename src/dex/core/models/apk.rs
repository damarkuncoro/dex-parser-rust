use serde::Serialize;
use super::Dex;
use crate::dex::analysis::GlobalIntelligence;
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Serialize)]
pub struct Apk<'a> {
    pub dex_files: Vec<Dex<'a>>,
    pub dex_names: Vec<String>,
    /// Maps class names to the index of the DEX file that defines them.
    pub class_lookup: HashMap<String, usize>,
    /// APK-wide intelligence findings.
    pub intelligence: GlobalIntelligence,
    /// AndroidManifest.xml data if available.
    pub manifest: Option<Manifest>,
    /// resources.arsc data if available.
    pub resources: Option<crate::apk::resources::ResourceTable>,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct Manifest {
    pub package_name: String,
    pub permissions: Vec<String>,
    pub activities: Vec<String>,
    pub services: Vec<String>,
    pub receivers: Vec<String>,
    pub providers: Vec<String>,
}

impl<'a> Apk<'a> {
    pub fn new(dex_files: Vec<Dex<'a>>, dex_names: Vec<String>) -> Self {
        Self::new_full(dex_files, dex_names, None, None)
    }

    pub fn new_with_manifest(dex_files: Vec<Dex<'a>>, dex_names: Vec<String>, manifest: Option<Manifest>) -> Self {
        Self::new_full(dex_files, dex_names, manifest, None)
    }

    pub fn new_full(
        dex_files: Vec<Dex<'a>>,
        dex_names: Vec<String>,
        manifest: Option<Manifest>,
        resources: Option<crate::apk::resources::ResourceTable>
    ) -> Self {
        let class_lookup: HashMap<String, usize> = dex_files.par_iter().enumerate()
            .flat_map(|(i, dex)| {
                dex.class_defs.par_iter().map(move |class| (class.name.to_string(), i))
            })
            .collect();

        Self {
            dex_files,
            dex_names,
            class_lookup,
            intelligence: GlobalIntelligence::default(),
            manifest,
            resources,
        }
    }
}
