use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantityLeftModel {
    pub tour: i32,
    pub date: NaiveDate,
    pub label: String,
    pub size: Option<String>,
    pub quantity: Option<i64>,
}

impl QuantityLeftModel {
    pub fn new(
        tour: i32,
        date: NaiveDate,
        label: String,
        size: Option<String>,
        quantity: Option<i64>,
    ) -> Self {
        Self {
            tour,
            date,
            label,
            size,
            quantity,
        }
    }
}
