use crate::models::user::User;
use sqlx::Error;
use actix_web::web;

#[derive(Clone)]
pub struct UserRepository {
    app_state: web::Data<crate::AppState>,
}

impl UserRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM pfe.users WHERE user_id = $1", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        Ok(user)
    }
    pub async fn verify_user(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "UPDATE pfe.users SET is_verified = true WHERE user_id = $1 RETURNING *", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        Ok(user)
    }
    pub async fn revoke_user(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "DELETE FROM pfe.users WHERE user_id = $1 RETURNING *", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;   
        Ok(user)
    }
    pub async fn set_admin(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "UPDATE pfe.users SET is_admin = true WHERE user_id = $1 RETURNING *", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        Ok(user)
    }
    
}
