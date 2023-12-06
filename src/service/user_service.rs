use crate::{models::user::User, repository::user_repository::UserRepository};
use sqlx::Error;
extern crate bcrypt;
use bcrypt::{hash, verify};
const ROUND: u8 = 10; // Number of rounds to hash the password

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.get_item(id).await
    }

    pub async fn login(&self, email: &str, password: &str) -> Option<User> {

        let user_found = self.user_repo.get_user_by_email(email).await;


        match user_found{
            Ok(Some(user)) => {
                // User found, check password
                if verify(password, &user.password).unwrap() {
                    Some(user) // Password correct
                } else {
                    // Password incorrect
                    None
                }
            }
            Ok(None) => {
                // User not found
                None
            }
            Err(_) => {
                // Error occurred
                None
            }

        }
        

    }
}
