use serde::{Serialize, Deserialize};
use crate::analysis::core::models::RiskLevel;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestRule {
    pub category: String,
    pub description: String,
    pub required_permissions: Vec<String>,
    pub required_actions: Vec<String>,
    pub must_have_no_activities: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoringRule {
    pub category_pattern: String,
    pub points: f64,
    pub justification_prefix: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntelligenceRule {
    pub name: String,
    pub description: String,
    pub severity: RiskLevel,
    pub mitre_id: Option<String>,
    pub required_permissions: Vec<String>,
    pub required_behaviors: Vec<String>,
    pub requires_taint: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomScannerRule {
    pub category: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomBehavioralRule {
    pub category: String,
    pub indicator: String,
    pub description: String,
    pub mitre_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomHeuristicRule {
    pub category: String,
    pub pattern: String,
    pub min_len: usize,
}
