pub mod header;
pub mod class;
pub mod method;

use crate::dex::models::Dex;
use super::DexPrinter;

pub struct DexDumpPrinter;

impl DexPrinter for DexDumpPrinter {
    fn print(&self, dex: &Dex, path: &str) {
        println!("Processing '{}'...", path);
        println!("Opened '{}', DEX version '035'", path);
        header::print_header(dex);
        header::print_map_list(dex);
        for (i, class) in dex.classes.iter().enumerate() {
            class::print_class(dex, i, class);
        }
    }
}
