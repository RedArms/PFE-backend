use crate::models::item::Item;
use sqlx::postgres::PgPool;
use sqlx::Error;
use actix_web::web;

#[derive(Clone)]
pub struct ItemRepository {
    app_state: web::Data<crate::AppState>,
}

impl ItemRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        let item = sqlx::query_as!(Item, "SELECT item_id, label, size FROM pfe.items WHERE item_id = $1", id)
            .fetch_optional(&self.app_state.db_pool)
            .await?;

        Ok(item)
    }
}
