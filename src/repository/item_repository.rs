use crate::models::item::Item;
use actix_web::web;
use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(Clone)]
pub struct ItemRepository {
    app_state: web::Data<crate::AppState>,
}

impl ItemRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_item(&self, id: i32) -> Result<Option<Item>, Error> {
        let item = sqlx::query_as!(
            Item,
            "SELECT item_id, label, size FROM pfe.items WHERE item_id = $1",
            id
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(item)
    }

    pub async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        let items = sqlx::query_as!(Item, "SELECT item_id, label, size FROM pfe.items")
            .fetch_all(&self.app_state.db_pool)
            .await?;

        Ok(items)
    }

    pub async fn create_item(&self, item: Item) -> Result<Item, Error> {
        let item = sqlx::query_as!(
            Item,
            "INSERT INTO pfe.items (label, size) VALUES ($1, $2) RETURNING item_id, label, size",
            item.label,
            item.size
        )
        .fetch_one(&self.app_state.db_pool)
        .await?;

        Ok(item)
    }
}
