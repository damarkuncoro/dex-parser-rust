use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Proto<'a> {
    pub shorty: &'a str,
    pub return_type: &'a str,
    pub parameters: Vec<&'a str>,
}
