use crate::dex::core::models::Apk;
use crate::analysis::core::traits::ApkAnalyzer;
use crate::analysis::core::models::{GlobalIntelligence, CallSite};
use crate::analysis::forensics::engine::ManifestAnalyzer;
use crate::analysis::global::intelligence::IntelligenceEngine;
use std::collections::{HashMap, HashSet};
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
        let sensitive_methods: HashSet<String> = intel.behavioral_indicators.iter()
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

                local_intel.behavioral_indicators.extend(dex.analysis.sensitive_indicators.clone());

                let helper = |target_map: &mut HashMap<String, Vec<CallSite>>, source_map: &HashMap<String, Vec<String>>| {
                    for (caller_sig, targets) in source_map {
                        // Minimal parsing of signature
                        let class_name = caller_sig.split("->").next().unwrap_or(caller_sig).to_string();
                        let method_part = caller_sig.split("->").nth(1).unwrap_or("");
                        let method_name = method_part.split('(').next().unwrap_or(method_part).to_string();

                        let site = CallSite {
                            dex_name: current_dex_name.clone(),
                            class_name,
                            method_name,
                            method_signature: caller_sig.clone(),
                        };

                        for target in targets {
                            target_map.entry(target.clone()).or_default().push(site.clone());
                        }
                    }
                };

                helper(&mut local_intel.cross_dex_calls, &dex.analysis.xrefs.method_to_methods);
                helper(&mut local_intel.cross_dex_fields, &dex.analysis.xrefs.method_to_fields);
                helper(&mut local_intel.cross_dex_strings, &dex.analysis.xrefs.method_to_strings);
                helper(&mut local_intel.cross_dex_types, &dex.analysis.xrefs.method_to_types);

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
