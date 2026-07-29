use crate::analysis::core::models::{AnalysisReport, RiskAssessment, RiskLevel};
use crate::analysis::core::config::AnalysisConfig;

pub struct ScoringEngine;

impl ScoringEngine {
    pub fn assess(report: &AnalysisReport, config: &AnalysisConfig) -> RiskAssessment {
        let mut score: f64 = 0.0;
        let mut justifications = Vec::new();

        let all_indicators = report.manifest_indicators.iter().chain(report.sensitive_indicators.iter());

        for rule in &config.scoring_rules {
            let mut matched = false;
            for indicator in all_indicators.clone() {
                if indicator.category.contains(&rule.category_pattern) {
                    if !matched {
                        score += rule.points;
                        justifications.push(format!("{}: {}", rule.justification_prefix, indicator.content));
                        matched = true;
                    }
                }
            }
        }

        // Evaluate Forensic Gaps (Packers)
        let suspicious_gaps = report.forensic_gaps.iter().filter(|g| g.is_suspicious).count();
        if suspicious_gaps > 0 {
            score += 2.5;
            justifications.push(format!("Likely Packed/Obfuscated ({} high-entropy gaps)", suspicious_gaps));
        }

        let final_score = score.min(10.0);

        let level = match final_score {
            s if s >= 8.5 => RiskLevel::Critical,
            s if s >= 6.0 => RiskLevel::High,
            s if s >= 3.5 => RiskLevel::Medium,
            s if s >= 1.0 => RiskLevel::Low,
            _ => RiskLevel::Safe,
        };

        RiskAssessment {
            score: final_score,
            level,
            justifications,
        }
    }
}
