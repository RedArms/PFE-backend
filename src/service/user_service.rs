use crate::{models::user::User, repository::user_repository::UserRepository};
use sqlx::Error;

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

    pub async fn verify_user(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.verify_user(id).await
    }
}
