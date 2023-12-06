// src/repository/item_repository.rs

use sqlx::{postgres::PgPool, Error};

use crate::models::item::Item;
use crate::models::user::User;

#[derive(Clone)]
pub struct pgsqlConn {
    pool: PgPool,
}

impl pgsqlConn {

    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        let item = sqlx::query_as!(Item, "SELECT id, name FROM items WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(item)
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User,"SELECT \"id\" , \"last_name\" , \"first_name\" , email, \"is_admin\" , \"is_verified\" , telephone FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

}
