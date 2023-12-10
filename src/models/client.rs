use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub client_id: i32,
    pub name: String,
    pub address: String,
    pub tour: Option<i32>,
}

impl Client {
    pub fn new(client_id: i32, name: &str, address: &str, tour: Option<i32>) -> Self {
        Client {
            client_id,
            name: name.to_string(),
            address: address.to_string(),
            tour,
        }
    }
}
