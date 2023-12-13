use crate::models::client::Client;
use crate::models::item::Item;
use crate::models::regular_order::RegularOrder;
use crate::models::regular_order_line::RegularOrderLine;
use crate::routes::items;
use actix_web::web;
use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(Clone)]
pub struct ClientRepository {
    app_state: web::Data<crate::AppState>,
}

impl ClientRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_all_clients(&self) -> Result<Vec<Client>, Error> {
        let client = sqlx::query_as!(
            Client,
            "SELECT client_id, name, address, tour FROM pfe.clients"
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(client)
    }

    pub async fn add_client(&self, client: Client) -> Result<Client, Error> {
        let client = sqlx::query_as!(Client, "INSERT INTO pfe.clients (name, address, tour) VALUES ($1, $2, $3) RETURNING client_id, name, address, tour", client.name, client.address, client.tour)
            .fetch_one(&self.app_state.db_pool)
            .await?;

        let items: Vec<Item> = sqlx::query_as!(Item, "SELECT item_id, label, size FROM pfe.items")
            .fetch_all(&self.app_state.db_pool)
            .await?;

        for item in items {
            sqlx::query!(
                "INSERT INTO pfe.client_lines (client, item, quantity) VALUES ($1, $2, 0)",
                client.client_id.unwrap(),
                item.item_id
            )
            .execute(&self.app_state.db_pool)
            .await?;
        }

        Ok(client)
    }

    pub async fn update_client(&self, id: i32, client: Client) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE pfe.clients SET name = $1, address = $2, tour = $3 WHERE client_id = $4",
            client.name,
            client.address,
            client.tour,
            id
        )
        .execute(&self.app_state.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_order(&self, id: i32) -> Result<RegularOrder, Error> {
        let regular_order = sqlx::query_as!(RegularOrderLine, "SELECT i.item_id, i.label, i.size, cl.quantity FROM pfe.client_lines cl JOIN pfe.items i ON cl.item = i.item_id WHERE cl.client = $1", id)
            .fetch_all(&self.app_state.db_pool)
            .await?;

        Ok(RegularOrder::new(regular_order))
    }

    pub async fn update_order(&self, id: i32, order: RegularOrder) -> Result<(), Error> {
        for line in order.regular_order_lines {
            sqlx::query!(
                "UPDATE pfe.client_lines SET quantity = $1 WHERE client = $2 AND item = $3",
                line.quantity,
                id,
                line.item_id
            )
            .execute(&self.app_state.db_pool)
            .await?;
        }

        Ok(())
    }

    pub async fn delete_client(&self, id: i32) -> Result<(), Error> {
        let order_ids = sqlx::query!("SELECT order_id FROM pfe.orders WHERE client = $1", id)
            .fetch_all(&self.app_state.db_pool)
            .await?;

        for record in order_ids {
            sqlx::query!("DELETE FROM pfe.boxes WHERE order_id = $1", record.order_id)
                .execute(&self.app_state.db_pool)
                .await?;

            sqlx::query!("DELETE FROM pfe.orders WHERE client = $1", id)
                .execute(&self.app_state.db_pool)
                .await?;
        }

        sqlx::query!("DELETE FROM pfe.client_lines WHERE client = $1", id)
            .execute(&self.app_state.db_pool)
            .await?;

        sqlx::query!("DELETE FROM pfe.clients WHERE client_id = $1", id)
            .execute(&self.app_state.db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_all_clients_tours(&self, id: i32) -> Result<Vec<Client>, Error> {
        let clients = sqlx::query_as!(
            Client,
            "SELECT client_id, name, address, tour FROM pfe.clients WHERE tour = $1",
            id
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(clients)
    }
}
