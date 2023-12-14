use crate::models::regular_order_line::RegularOrderLine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegularOrder {
    pub regular_order_lines: Vec<RegularOrderLine>,
}

impl RegularOrder {
    pub fn new(regular_order_lines: Vec<RegularOrderLine>) -> Self {
        Self {
            regular_order_lines,
        }
    }
}
