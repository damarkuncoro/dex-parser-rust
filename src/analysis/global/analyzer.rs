use crate::dex::core::models::Apk;
use crate::analysis::core::traits::ApkAnalyzer;
use crate::analysis::core::utils::{Reference, ReferenceExtractor};
use crate::analysis::core::models::{GlobalIntelligence, CallSite};
use crate::analysis::forensics::engine::ManifestAnalyzer;
use crate::analysis::global::intelligence::IntelligenceEngine;
use rayon::prelude::*;

pub struct GlobalAnalyzer;

impl ApkAnalyzer for GlobalAnalyzer {
    type Output = GlobalIntelligence;
    fn analyze(&self, apk: &Apk, dex_names: &[String]) -> Self::Output {
        let mut intel = GlobalIntelligence::build(apk, dex_names);

        // 1. Resolve Resource IDs from code
        if let Some(res_table) = &apk.resources {
            for dex in &apk.dex_files {
                for &id in &dex.analysis.potential_resource_ids {
                    if let Some(name) = res_table.id_map.get(&id) {
                        intel.resolved_resources.insert(id, name.clone());
                    }
                }
            }
        }

        // 2. Add Manifest Analysis to global indicators
        let config = apk.dex_files.first().map(|d| &d.analysis_config).cloned().unwrap_or_default();
        if let Some(manifest) = &apk.manifest {
            let manifest_results = ManifestAnalyzer::analyze(manifest, &config);
            intel.behavioral_indicators.extend(manifest_results);
        }

        // 3. Automated Intelligence Tagging
        IntelligenceEngine::tag_apk(apk, &mut intel, &config);

        intel.deduplicate();

        // 4. Optimization for UI: Filter XREFs to only show interesting ones
        let sensitive_methods: std::collections::HashSet<String> = intel.behavioral_indicators.iter()
            .map(|i| i.content.clone())
            .collect();

        intel.cross_dex_calls.retain(|k, _| {
            sensitive_methods.iter().any(|sm| k.contains(sm) || sm.contains(k))
        });

        intel
    }
}

impl GlobalIntelligence {
    pub fn build(apk: &Apk, dex_names: &[String]) -> Self {
        let mut intel = apk.dex_files.par_iter().enumerate()
            .map(|(i, dex)| {
                let mut local_intel = Self::default();
                let current_dex_name = dex_names.get(i).cloned()
                    .unwrap_or_else(|| format!("classes{}.dex", if i == 0 { "".to_string() } else { (i + 1).to_string() }));

                local_intel.global_security_summary.total_suspicious_gaps = dex.analysis.stats.suspicious_gap_count;
                local_intel.global_security_summary.total_sensitive_indicators = dex.analysis.stats.sensitive_count;
                local_intel.global_security_summary.total_spec_violations = dex.analysis.stats.spec_violation_count;
                local_intel.global_security_summary.total_dead_code = dex.analysis.stats.dead_code_count;
                local_intel.global_security_summary.potentially_packed = dex.analysis.stats.suspicious_gap_count > 0;

                // 1. Pull already analyzed behavioral indicators (Taint, Crypto, etc.)
                local_intel.behavioral_indicators.extend(dex.analysis.sensitive_indicators.clone());

                // 2. Build Cross-DEX references
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
