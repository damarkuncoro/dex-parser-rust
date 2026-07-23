pub mod json;
pub mod text;

use crate::dex::models::Dex;
use std::io::Write;

/// Trait for defining different output strategies for the parsed DEX data.
pub trait DexPrinter {
    /// Prints or exports the parsed `Dex` information to a writer.
    fn print(&self, dex: &Dex, path: &str, writer: &mut dyn Write) -> std::io::Result<()>;
}
