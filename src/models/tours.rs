use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Tours {
    pub tour_id: i32,
    pub geo_zone: String,
    pub delivery_person : i32,
}

impl Tours {
    pub fn new(tour_id: i32, geo_zone: &str,delivery_person:i32) -> Self {
        Self {
            tour_id : tour_id,
            geo_zone: geo_zone.to_string(),
            delivery_person: delivery_person,
        }
    }
}
