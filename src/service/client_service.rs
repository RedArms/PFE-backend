use crate::models::boxe::Boxe;
use crate::models::client::Client;
use crate::models::regular_order::RegularOrder;
use crate::repository::boxe_repository::BoxeRepository;
use crate::repository::client_repository::ClientRepository;
use crate::repository::order_repository::OrderRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct ClientService {
    client_repo: ClientRepository,
    boxe_repo: BoxeRepository,
    order_repo: OrderRepository,
}

impl ClientService {
    pub fn new(
        client_repo: ClientRepository,
        boxe_repo: BoxeRepository,
        order_repo: OrderRepository,
    ) -> Self {
        Self {
            client_repo,
            boxe_repo,
            order_repo,
        }
    }

    pub async fn get_all_clients(&self) -> Result<Vec<Client>, Error> {
        self.client_repo.get_all_clients().await
    }

    pub async fn add_client(&self, client: Client) -> Result<Client, Error> {
        self.client_repo.add_client(client).await
    }

    pub async fn update_client(&self, id: i32, client: Client) -> Result<(), Error> {
        self.client_repo.update_client(id, client).await
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

    pub async fn get_all_client_by_tour(&self, id: i32) -> Result<Vec<Client>, Error> {
        self.client_repo.get_all_clients_tours(id).await
    }

    pub async fn get_all_boxes_client(
        &self,
        id: i32,
        tour_day: i32,
        date: String,
    ) -> Result<Vec<Boxe>, Error> {
        let id_order = self.order_repo.get_order_id(id, tour_day, date).await?;
        println!("id_order : {}", id_order);
        if id_order == 0 {
            return Ok(Vec::new());
        }
        self.boxe_repo.get_all_boxes(id_order).await
    }
}
