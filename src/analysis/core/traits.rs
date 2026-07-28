use crate::dex::core::models::Dex;

/// Base trait for all DEX analysis components.
pub trait DexAnalyzer {
    type Output;
    fn analyze(&self, dex: &Dex) -> Self::Output;
}

/// Base trait for APK-wide analysis.
pub trait ApkAnalyzer {
    type Output;
    fn analyze(&self, apk: &crate::dex::core::models::Apk, dex_names: &[String]) -> Self::Output;
}
