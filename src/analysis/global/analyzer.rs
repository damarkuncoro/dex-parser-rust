use crate::dex::core::models::Apk;
use crate::analysis::core::traits::ApkAnalyzer;
use crate::analysis::core::utils::{Reference, ReferenceExtractor};
use crate::analysis::core::models::{GlobalIntelligence, CallSite};
use crate::analysis::forensics::rules::BehaviorScanner;
use rayon::prelude::*;

pub struct GlobalAnalyzer;

impl ApkAnalyzer for GlobalAnalyzer {
    type Output = GlobalIntelligence;
    fn analyze(&self, apk: &Apk, dex_names: &[String]) -> Self::Output {
        GlobalIntelligence::build(apk, dex_names)
    }
}

impl GlobalIntelligence {
    pub fn build(apk: &Apk, dex_names: &[String]) -> Self {
        let config = apk.dex_files.first().map(|d| &d.analysis_config).cloned()
            .unwrap_or_default();

        let mut intel = apk.dex_files.par_iter().enumerate()
            .map(|(i, dex)| {
                let mut local_intel = Self::default();
                let current_dex_name = dex_names.get(i).cloned()
                    .unwrap_or_else(|| format!("classes{}.dex", if i == 0 { "".to_string() } else { (i + 1).to_string() }));

                local_intel.global_security_summary.total_suspicious_gaps = dex.analysis.stats.suspicious_gap_count;
                local_intel.global_security_summary.total_sensitive_indicators = dex.analysis.stats.sensitive_count;
                local_intel.global_security_summary.potentially_packed = dex.analysis.stats.suspicious_gap_count > 0;

                for class in &dex.class_defs {
                    let all_methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
                    for method in all_methods {
                        if let Some(code) = &method.code {
                            let site = CallSite {
                                dex_name: current_dex_name.clone(),
                                class_name: class.name.clone(),
                                method_name: method.name.clone(),
                                method_signature: method.signature.clone(),
                            };

                            for ins in &code.instructions {
                                if let Some(reference) = ReferenceExtractor::extract(ins) {
                                    if let Some(found) = BehaviorScanner::check_reference(&reference, &config) {
                                        local_intel.behavioral_indicators.push(found);
                                    }

                                    match reference {
                                        Reference::Method(target) => {
                                            local_intel.cross_dex_calls.entry(target.to_string())
                                                .or_default().push(site.clone());
                                        }
                                        Reference::Field(target) => {
                                            local_intel.cross_dex_fields.entry(target.to_string())
                                                .or_default().push(site.clone());
                                        }
                                        Reference::String(target) => {
                                            local_intel.cross_dex_strings.entry(target.to_string())
                                                .or_default().push(site.clone());
                                        }
                                        Reference::Type(target) => {
                                            local_intel.cross_dex_types.entry(target.to_string())
                                                .or_default().push(site.clone());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
                local_intel
            })
            .reduce(Self::default, |mut a, b| {
                a.merge(b);
                a
            });

        intel.deduplicate();
        intel
    }
}
