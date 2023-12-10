use crate::models::{client::Client, regular_order::RegularOrder, regular_order_line::RegularOrderLine};
use sqlx::Error;
use crate::repository::client_repository::ClientRepository;

#[derive(Clone)]
pub struct ClientService {
    client_repo: ClientRepository,
}

impl ClientService {
    pub fn new(client_repo: ClientRepository) -> Self {
        Self { client_repo }
    }

    pub async fn get_all_clients(&self) -> Result<Vec<Client>, Error> {
        self.client_repo.get_all_clients().await
    }

    pub async fn add_client(&self, client: Client) -> Result<Client, Error> {
        self.client_repo.add_client(client).await
    }
    
    pub async fn get_order(&self, id: i32) -> Result<RegularOrder, Error> {
        self.client_repo.get_order(id).await
    }

    pub async fn update_order(&self, id: i32, order: RegularOrder) -> Result<(), Error> {
        self.client_repo.update_order(id, order).await
    }

    pub async fn delete_client(&self, id: i32) -> Result<(), Error> {
        self.client_repo.delete_client(id).await
    }
}
