use crate::dex::core::models::Apk;
use crate::analysis::core::models::{AnalysisTag, GlobalIntelligence};
use crate::analysis::core::config::AnalysisConfig;

pub struct IntelligenceEngine;

impl IntelligenceEngine {
    pub fn tag_apk(apk: &Apk, intel: &mut GlobalIntelligence, config: &AnalysisConfig) {
        let mut tags = Vec::new();

        // Helper closures for checking context
        let has_permission = |p: &str| {
            apk.manifest.as_ref().map_or(false, |m| m.permissions.iter().any(|perm| perm.contains(p)))
        };

        let has_behavior = |b: &str| {
            intel.behavioral_indicators.iter().any(|ind| ind.category.contains(b))
        };

        let has_sink_leak = intel.behavioral_indicators.iter().any(|ind| ind.category.contains("Sensitive Taint"));

        for rule in &config.intelligence_rules {
            let perm_match = rule.required_permissions.is_empty() ||
                             rule.required_permissions.iter().all(|p| has_permission(p));

            let behavior_match = rule.required_behaviors.is_empty() ||
                                 rule.required_behaviors.iter().all(|b| has_behavior(b));

            let taint_match = !rule.requires_taint || has_sink_leak;

            if perm_match && behavior_match && taint_match {
                tags.push(AnalysisTag {
                    name: rule.name.clone(),
                    description: rule.description.clone(),
                    severity: rule.severity.clone(),
                    mitre_id: rule.mitre_id.clone(),
                });
            }
        }

        intel.analysis_tags = tags;
    }
}
