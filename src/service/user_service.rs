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

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        self.user_repo.get_user_by_email(email).await
    }
    pub async fn login(&self, email: &str, password: &str) -> Option<User> {
        let user_found = self.user_repo.get_user_by_email(email).await;

        match user_found {
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

    pub async fn register(&self, user: User) -> Result<i32, Error> {
        let hashed_password = hash(&user.password, ROUND.into()).unwrap();
        let user = User {
            password: hashed_password,
            ..user
        };
        match self.user_repo.create_user(user).await {
            Ok(user_id) => Ok(user_id),
            Err(_) => Err(Error::RowNotFound),
        }
    }
    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        self.user_repo.get_all_users().await
    }
    pub async fn verify_user(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.verify_user(id).await
    }

    pub async fn revoke_user(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.revoke_user(id).await
    }

    pub async fn set_admin(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.set_admin(id).await
    }
}
