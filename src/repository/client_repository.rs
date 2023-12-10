use crate::models::client::Client;
use actix_web::web;
use sqlx::Error;

#[derive(Clone)]
pub struct ClientRepository {
    app_state: web::Data<crate::AppState>,
}

impl ClientRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
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
