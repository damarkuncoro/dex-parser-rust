use serde::{Serialize};

#[derive(Serialize,  Clone)]
pub struct Field<'a> {
    pub class: &'a str,
    pub type_name: &'a str,
    pub name: &'a str,
}
