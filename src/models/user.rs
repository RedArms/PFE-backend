use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_verified: bool,
    pub telephone: String,
}

impl User {
    pub fn new(id: i32, last_name: &str,
               first_name: &str, email: &str,
               is_admin:bool, is_verified:bool,
               telephone:&str) -> Self {
        Self {
            id,
            last_name: last_name.to_string(),
            first_name: first_name.to_string(),
            email: email.to_string(),
            is_admin,
            is_verified,
            telephone: telephone.to_string(),
        }

        
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {} {} {} {} {}", self.id, self.last_name, self.first_name, self.email, self.is_admin, self.is_verified, self.telephone)
    }


}
