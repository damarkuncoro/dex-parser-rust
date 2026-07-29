use crate::dex::core::models::Manifest;
use crate::analysis::core::models::ScanResult;
use crate::analysis::core::config::AnalysisConfig;

pub struct ManifestAnalyzer;

impl ManifestAnalyzer {
    pub fn analyze(manifest: &Manifest, config: &AnalysisConfig) -> Vec<ScanResult> {
        let mut results = Vec::new();

        for rule in &config.manifest_rules {
            let mut matched = true;

            // Check permissions
            for perm in &rule.required_permissions {
                if !manifest.permissions.iter().any(|p| p.contains(perm)) {
                    matched = false;
                    break;
                }
            }
            if !matched { continue; }

            // Check actions in intent filters
            for action in &rule.required_actions {
                let mut found_action = false;
                let components = manifest.activities.iter()
                    .chain(manifest.services.iter())
                    .chain(manifest.receivers.iter());

                for comp in components {
                    for filter in &comp.intent_filters {
                        if filter.actions.iter().any(|a| a.contains(action)) {
                            found_action = true;
                            break;
                        }
                    }
                    if found_action { break; }
                }
                if !found_action {
                    matched = false;
                    break;
                }
            }
            if !matched { continue; }

            // Check stealth
            if rule.must_have_no_activities && !manifest.activities.is_empty() {
                continue;
            }

            results.push(ScanResult {
                category: rule.category.clone(),
                content: rule.description.clone(),
                details: None,
            });
        }

        results
    }
}
