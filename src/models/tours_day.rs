use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToursDay {
    pub tour_id: i32,
    pub geo_zone: String,
    pub delivery_person: i32,
    pub jour: String,
}

impl ToursDay {
    pub fn new(tour_id: i32, geo_zone: &str, delivery_person: i32, jour: &str) -> Self {
        Self {
            tour_id: tour_id,
            geo_zone: geo_zone.to_string(),
            delivery_person: delivery_person,
            jour: jour.to_string(),
        }
    }
}