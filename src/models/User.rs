use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}