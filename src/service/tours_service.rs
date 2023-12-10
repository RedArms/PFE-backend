use crate::models::tours::Tours;
use crate::models::tours_day::ToursDay;
use crate::repository::tours_repository::ToursRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct ToursService {
    tours_repo: ToursRepository,
}

impl ToursService {
    pub fn new(tours_repo: ToursRepository) -> Self {
        Self { tours_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Tours>, Error> {
        self.tours_repo.get_all_tours().await
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
        return self
            .tours_repo
            .set_deliverer(date, tour, deliverer_id)
            .await;
    }

    pub async fn get_tours_day_avalaible(
        &self
    ) -> Result<Vec<ToursDay>, Error> {
        return self.tours_repo.get_tours_day_avalaible().await;
    }

    pub async fn get_by_id (&self, id: i32) -> Result<Option<Tours>, Error> {
        return self.tours_repo.get_by_id(id).await;
    }
}
