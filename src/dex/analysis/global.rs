use crate::dex::core::models::Apk;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Default, Debug)]
pub struct GlobalIntelligence {
    /// Maps a method signature to all its call sites across the entire APK.
    pub cross_dex_calls: HashMap<String, Vec<CallSite>>,
    /// Aggregated security findings from all DEX files.
    pub global_security_summary: GlobalSecuritySummary,
}

#[derive(Serialize, Debug, Clone)]
pub struct CallSite {
    pub dex_name: String,
    pub class_name: String,
    pub method_name: String,
}

#[derive(Serialize, Default, Debug)]
pub struct GlobalSecuritySummary {
    pub total_suspicious_gaps: usize,
    pub total_sensitive_indicators: usize,
    pub potentially_packed: bool,
}

pub struct GlobalAnalyzer;

impl GlobalIntelligence {
    pub fn build(apk: &Apk, dex_names: &[String]) -> Self {
        let mut intel = Self::default();
        let mut suspicious_gaps = 0;
        let mut sensitive_indicators = 0;

        for (i, dex) in apk.dex_files.iter().enumerate() {
            let current_dex_name = dex_names.get(i).cloned().unwrap_or_else(|| format!("classes{}.dex", i + 1));

            // 1. Aggregate Security Stats
            suspicious_gaps += dex.analysis.stats.suspicious_gap_count;
            sensitive_indicators += dex.analysis.stats.sensitive_count;

            // 2. Build APK-wide Cross-DEX Call Map
            for class in &dex.class_defs {
                let all_methods = class.direct_methods.iter().chain(class.virtual_methods.iter());
                for method in all_methods {
                    if let Some(code) = &method.code {
                        for ins in &code.instructions {
                            if let Some(called_method) = &ins.resolved_value {
                                if ins.name.contains("invoke") {
                                    intel.cross_dex_calls
                                        .entry(called_method.clone())
                                        .or_default()
                                        .push(CallSite {
                                            dex_name: current_dex_name.clone(),
                                            class_name: class.name.clone(),
                                            method_name: method.name.clone(),
                                        });
                                }
                            }
                        }
                    }
                }
            }
        }

        intel.global_security_summary = GlobalSecuritySummary {
            total_suspicious_gaps: suspicious_gaps,
            total_sensitive_indicators: sensitive_indicators,
            potentially_packed: suspicious_gaps > 0,
        };

        intel
    }
}
