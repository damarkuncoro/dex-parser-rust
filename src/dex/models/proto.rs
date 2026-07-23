use serde::Serialize;

#[derive(Serialize)]
pub struct Proto {
    pub shorty: String,
    pub return_type: String,
    pub parameters: Vec<String>,
}
