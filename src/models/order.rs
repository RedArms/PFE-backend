use serde::{Deserialize, Serialize};
use sqlx_core::types::chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_id: i32,
    pub client: i32,
    pub status: Option<String>,
    pub tour: i32,
    pub date: NaiveDate,
}

impl Order {
    pub fn new(
        order_id: i32,
        client: i32,
        status: Option<&str>,
        tour: i32,
        date: NaiveDate,
    ) -> Self {
        Self {
            order_id,
            client,
            status: status.map(|s| s.to_string()),
            tour,
            date,
        }
    }
}
