use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForensicConfig {
    pub entropy_threshold: f64,
    pub gap_length_threshold: usize,
    pub min_string_length: usize,
    pub suspicious_string_length: usize,
}
