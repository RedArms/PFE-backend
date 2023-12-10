use serde::{Serialize, Deserialize};
use crate::models::regular_order_line::RegularOrderLine;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegularOrder {
    pub regular_order_lines : Vec<RegularOrderLine>,
}

impl RegularOrder {
    pub fn new(regular_order_lines: Vec<RegularOrderLine>) -> Self {
        Self {
            regular_order_lines,
        }
    }
}
