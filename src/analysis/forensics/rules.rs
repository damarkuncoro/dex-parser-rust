use crate::analysis::core::models::ScanResult;
use crate::analysis::core::utils::Reference;
use crate::analysis::core::config::AnalysisConfig;

pub struct BehaviorScanner;

impl BehaviorScanner {
    pub fn check_reference(reference: &Reference, config: &AnalysisConfig) -> Option<ScanResult> {
        let target = match reference {
            Reference::Method(t) => t,
            Reference::Type(t) => t,
            _ => return None,
        };

        for rule in &config.behavioral_rules {
            if target.contains(&rule.indicator) {
                let category = if let Some(perm) = &rule.required_permission {
                    format!("Behavior: {} (Requires {})", rule.category, perm)
                } else {
                    format!("Behavior: {}", rule.category)
                };

                return Some(ScanResult {
                    category,
                    content: format!("{}: {}", rule.description, target),
                    details: None,
                });
            }
        }
        None
    }
}
