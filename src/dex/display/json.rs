use crate::dex::models::Dex;
use super::DexPrinter;

pub struct JsonPrinter;

impl DexPrinter for JsonPrinter {
    fn print(&self, dex: &Dex, _path: &str) {
        match serde_json::to_string_pretty(dex) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Error serializing to JSON: {}", e),
        }
    }
}
