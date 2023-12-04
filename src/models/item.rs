
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

impl Item {
    pub fn new(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}
