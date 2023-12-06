use crate::models::user::User;
use actix_web::web;
use sqlx::postgres::PgPool;
use sqlx::Error;

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
}
