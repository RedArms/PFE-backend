use crate::models::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegularOrderLine {
    pub item_id: i32,
    pub label: String,
    pub size: Option<String>,
    pub quantity: i32,
}

impl RegularOrderLine {
    pub fn new(item_id: i32, label: &str, size: Option<&str>, quantity: i32) -> Self {
        Self {
            item_id,
            label: label.to_string(),
            size: size.map(|s| s.to_string()),
            quantity,
        }
    }
}
