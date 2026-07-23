pub mod text;
pub mod json;

use crate::dex::models::Dex;

pub trait DexPrinter {
    fn print(&self, dex: &Dex, path: &str);
}
