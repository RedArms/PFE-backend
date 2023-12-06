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

    pub async fn login(&self, email: &str, password: &str) -> Result<User, Error> {

        let user_found = self.user_repo.get_user_by_email(email).await;

        match user_found {
            Ok(user) => {
                match user {
                    Some(user) => {
                       // Bcrypt::verify(password, &user.password);
                       



                        


                        if user.password == password {
                            Ok(user)
                        } else {
                            Err(Error::RowNotFound)
                        }
                    }
                    None => Err(Error::RowNotFound),
                }
            }
            Err(e) => Err(e),
        }

    
    }
}
