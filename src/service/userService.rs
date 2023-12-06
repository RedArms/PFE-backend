use crate::{models::user::User, repository::userRepo::UserRepo};
use sqlx::Error;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepo,
}

impl UserService {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, Error> {
        self.user_repo.get_item(id).await
    }
}
