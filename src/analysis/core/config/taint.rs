use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaintConfig {
    pub enabled: bool,
    /// APIs that produce sensitive data (e.g., getDeviceId)
    pub sources: Vec<String>,
    /// APIs that exfiltrate data (e.g., URL.openConnection, SMS.send)
    pub sinks: Vec<String>,
}
