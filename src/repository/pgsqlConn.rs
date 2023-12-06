use sqlx::{postgres::PgPool, Error};
use std::sync::{Arc, Mutex};

use crate::models::item::Item;
use crate::models::user::User;

#[derive(Clone)]
pub struct pgsqlConn {
    pool: Option<Arc<Mutex<PgPool>>>,
    database_url: String,
}

impl pgsqlConn {
    pub fn new(database_url: &str) -> Self {
        Self {
            pool: None,
            database_url: database_url.to_string(),
        }
    }

    pub async fn get_pool(&mut self) -> Result<Arc<Mutex<PgPool>>, Error> {
        if let Some(pool) = self.pool.clone() {
            Ok(pool)
        } else {
            let new_pool = Arc::new(Mutex::new(PgPool::connect(&self.database_url).await?));
            self.pool = Some(new_pool.clone());
            Ok(new_pool)
        }
    }
}