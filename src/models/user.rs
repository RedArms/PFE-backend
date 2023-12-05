use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub lastName: String,
    pub firstName: String,
    pub email: String,
    pub isAdmin: bool,
    pub isVerified: bool,
    pub telephone: String,
}

impl User {
    pub fn new(id: i32, lastName: &str,
                firstName: &str,email: &str,
                isAdmin:bool,isVerified:bool,
                telephone:&str) -> Self {
        Self {
            id,
            lastName: lastName.to_string(),
            firstName: firstName.to_string(),
            email: email.to_string(),
            isAdmin,
            isVerified,
            telephone: telephone.to_string(),
        }
    }


}
