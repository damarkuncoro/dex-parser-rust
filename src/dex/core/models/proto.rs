use serde::{Serialize};

#[derive(Serialize,  Clone)]
pub struct Proto<'a> {
    pub shorty: String,
    pub return_type: String,
    pub parameters: Vec<String>,
    #[serde(skip)] pub _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Proto<'a> {
    pub fn new(shorty: String, return_type: String) -> Self {
        Self {
            shorty,
            return_type,
            parameters: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
}
