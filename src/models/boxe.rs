use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Boxe {
    pub order_id: i32,
    pub item: i32,
    pub quantity: i32,
    pub delivered_qty: i32,
    pub box_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Boxe_DTO {
    pub order_id: i32,
    pub item_id: i32,
    pub name: String,
    pub size: Option<String>,
    pub delivered_qty: i32,
    pub quantity: i32,
    pub box_status: Option<String>,
}
impl Boxe {
    pub fn new(
        order_id: i32,
        item: i32,
        quantity: i32,
        delivered_qty: i32,
        box_status: Option<&str>,
    ) -> Self {
        Boxe {
            order_id,
            item,
            quantity,
            delivered_qty,
            box_status: box_status.map(|s| s.to_string()),
        }
    }
}

impl Boxe_DTO {
    pub fn new(
        order_id: i32,
        item_id: i32,
        name: String,
        size: Option<String>,
        delivered_qty: i32,
        quantity: i32,
        box_status: Option<String>,
    ) -> Self {
        Boxe_DTO {
            order_id,
            item_id,
            name,
            size,
            delivered_qty,
            quantity,
            box_status,
        }
    }
}
