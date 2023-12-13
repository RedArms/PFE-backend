use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_id: Option<i32>,
    pub label: String,
    pub size: Option<String>,
}

impl Item {
    pub fn new(item_id: i32, label: &str, size: Option<&str>) -> Self {
        Self {
            item_id: Some(item_id),
            label: label.to_string(),
            size: size.map(|s| s.to_string()),
        }
    }
}
