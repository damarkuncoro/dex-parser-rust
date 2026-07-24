use serde::{Serialize};

#[derive(Serialize, Clone)]
pub struct Field<'a> {
    pub class: String,
    pub type_name: String,
    pub name: String,
    #[serde(skip)] pub _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Field<'a> {
    pub fn new(class: String, type_name: String, name: String) -> Self {
        Self {
            class,
            type_name,
            name,
            _marker: std::marker::PhantomData,
        }
    }
}
