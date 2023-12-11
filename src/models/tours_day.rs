use serde::{Deserialize, Serialize};
use sqlx_core::types::chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToursDay {
    pub tour: i32,
    pub delivery_person: Option<i32>,
    pub date: NaiveDate,
}

impl ToursDay {
    pub fn new(tour: i32, delivery_person: Option<i32>, date: NaiveDate) -> Self {
        Self {
            tour: tour,
            delivery_person: delivery_person,
            date: date,
        }
    }
}
