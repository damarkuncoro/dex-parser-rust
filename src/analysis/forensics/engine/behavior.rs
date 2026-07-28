use crate::dex::core::models::Class;
use crate::analysis::core::models::ScanResult;
use crate::analysis::core::utils::ReferenceExtractor;
use crate::analysis::forensics::rules::BehaviorScanner;
use crate::analysis::core::config::AnalysisConfig;
use rayon::prelude::*;

pub struct BehaviorAnalyzer;

impl BehaviorAnalyzer {
    pub fn analyze(classes: &[Class], config: &AnalysisConfig) -> Vec<ScanResult> {
        let mut results: Vec<ScanResult> = classes.par_iter()
            .flat_map(|class| {
                let mut local = Vec::new();
                let methods = class.direct_methods.iter().chain(class.virtual_methods.iter());

                for method in methods {
                    if let Some(code) = &method.code {
                        for ins in &code.instructions {
                            if let Some(reference) = ReferenceExtractor::extract(ins) {
                                if let Some(found) = BehaviorScanner::check_reference(&reference, config) {
                                    local.push(found);
                                }
                            }
                        }
                    }
                }
                local
            })
            .collect();

        results.sort_by(|a, b| a.content.cmp(&b.content));
        results.dedup_by(|a, b| a.content == b.content);

        results
    }
}
