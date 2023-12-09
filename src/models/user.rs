use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub is_admin: Option<bool>,
    pub is_verified: Option<bool>,
    pub is_delivery_person: Option<bool>,
    pub phone: String,
    pub password: String,
}

impl User {
    pub fn new(
        user_id: i32,
        last_name: &str,
        first_name: &str,
        email: &str,
        is_admin: bool,
        is_verified: bool,
        phone: &str,
        password: &str,
        is_delivery_person: bool,
    ) -> Self {
        Self {
            user_id: Some(user_id),
            last_name: last_name.to_string(),
            first_name: first_name.to_string(),
            email: email.to_string(),
            is_admin: Some(is_admin),
            is_verified: Some(is_verified),
            phone: phone.to_string(),
            password: password.to_string(),
            is_delivery_person: Some(is_delivery_person),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:?} {} {} {} {:?} {:?} {}",
            self.user_id,
            self.last_name,
            self.first_name,
            self.email,
            self.is_admin,
            self.is_verified,
            self.phone
        )
    }
}
