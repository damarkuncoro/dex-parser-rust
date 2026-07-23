pub mod rules;

use crate::dex::models::header::RawHeader;
use crate::dex::error::DexError;
use self::rules::{MagicRule, ChecksumRule, OffsetBoundsRule};

/// Trait for individual validation rules.
pub trait ValidationRule {
    fn validate(&self, buffer: &[u8], header: &RawHeader) -> Result<(), DexError>;
}

/// Orchestrator for running multiple validation rules.
pub struct DexValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Default for DexValidator {
    fn default() -> Self {
        Self {
            rules: vec![
                Box::new(MagicRule),
                Box::new(ChecksumRule),
                Box::new(OffsetBoundsRule),
            ],
        }
    }
}

impl DexValidator {
    pub fn new() -> Self { Self::default() }

    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    pub fn validate(&self, buffer: &[u8], header: &RawHeader) -> Result<(), DexError> {
        for rule in &self.rules {
            rule.validate(buffer, header)?;
        }
        Ok(())
    }
}
