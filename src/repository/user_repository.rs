use crate::models::user::User;
use actix_web::web;
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

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM pfe.users WHERE email = $1", email)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        Ok(user)
    }
    pub async fn verify_user(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE pfe.users SET is_verified = true WHERE user_id = $1 RETURNING *",
            id
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM pfe.users")
            .fetch_all(&self.app_state.db_pool)
            .await?;

        Ok(users)
    }

    pub async fn create_user(&self, user: User) -> Result<i32, Error> {
        let result = sqlx::query!(
            "INSERT INTO pfe.users (email, password, first_name, last_name, phone) VALUES ($1, $2, $3, $4, $5) returning user_id",
            user.email,
            user.password,
            user.first_name,
            user.last_name,
            user.phone
        )
        .fetch_one(&self.app_state.db_pool)
        .await?;

        // `user_id` sera automatiquement extrait du résultat SQL
        let user_id = result.user_id;

        Ok(user_id)
    }
    pub async fn revoke_user(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            "DELETE FROM pfe.users WHERE user_id = $1 RETURNING *;",
            id
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;
        Ok(user)
    }
    pub async fn set_admin(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE pfe.users SET is_admin = true WHERE user_id = $1 RETURNING *",
            id
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(user)
    }
}
