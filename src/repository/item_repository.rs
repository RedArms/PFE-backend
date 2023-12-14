use crate::models::{client::Client, item::Item};
use actix_web::web;
use aws_config::imds::client;
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

        let clients: Vec<Client> = sqlx::query_as!(
            Client,
            "SELECT client_id, name, address, tour FROM pfe.clients"
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        for client in clients {
            sqlx::query!(
                "INSERT INTO pfe.client_lines (client, item, quantity) VALUES ($1, $2, 0)",
                client.client_id.unwrap(),
                item.item_id
            )
            .execute(&self.app_state.db_pool)
            .await?;
        }

        Ok(item)
    }
}
