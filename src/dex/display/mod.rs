pub mod json;
pub mod text;

use crate::dex::models::Dex;

/// Trait for defining different output strategies for the parsed DEX data.
pub trait DexPrinter {
    /// Prints or exports the parsed `Dex` information.
    ///
    /// # Arguments
    /// * `dex` - The parsed `Dex` model.
    /// * `path` - Original file path for display purposes.
    fn print(&self, dex: &Dex, path: &str);
}
