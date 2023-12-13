use crate::models::quantity_left_model::QuantityLeftModel;
use crate::models::tours::Tours;
use crate::models::tours_day::ToursDay;
use crate::repository::order_repository::OrderRepository;
use crate::repository::tours_repository::ToursRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct ToursService {
    tours_repo: ToursRepository,
    order_repo: OrderRepository,
}

impl ToursService {
    pub fn new(tours_repo: ToursRepository, order_repo: OrderRepository) -> Self {
        Self {
            tours_repo,
            order_repo,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Tours>, Error> {
        self.tours_repo.get_all_tours().await
    }

    pub async fn get_all_tours_day(&self) -> Result<Vec<ToursDay>, Error> {
        return self.tours_repo.get_all_tours_day().await;
    }

    pub async fn get_tours_today(&self) -> Result<Vec<ToursDay>, Error> {
        self.tours_repo.get_tours_today().await
    }

    pub async fn set_deliverer(
        &self,
        tour: i32,
        date: String,
        deliverer_id: i32,
    ) -> Result<u64, Error> {
        self.order_repo
            .set_state_delivering(date.clone(), tour)
            .await?;
        return self
            .tours_repo
            .set_deliverer(date.clone(), tour, deliverer_id)
            .await;
    }

    pub async fn get_tours_day_avalaible(&self) -> Result<Vec<ToursDay>, Error> {
        return self.tours_repo.get_tours_day_avalaible().await;
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Tours>, Error> {
        return self.tours_repo.get_by_id(id).await;
    }

    pub async fn get_tours_by_delivery_day(&self, date: String) -> Result<Vec<ToursDay>, Error> {
        println!("get_tours_by_delivery_day SEVICE");

        return self.tours_repo.get_tours_by_delivery_day(date).await;
    }

    pub async fn get_tours_for_deliverer(
        &self,
        deliverer_id: i32,
    ) -> Result<Option<ToursDay>, Error> {
        return self.tours_repo.get_tours_for_deliverer(deliverer_id).await;
    }

    pub async fn get_quatity_left(
        &self,
        date: String,
        tour: i32,
    ) -> Result<Vec<QuantityLeftModel>, Error> {
        return self.tours_repo.get_quatity_left(date, tour).await;
    }
}
