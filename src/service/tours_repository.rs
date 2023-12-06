use crate::models::tours::Tours;
use sqlx::Error;
use crate::repository::tours_repository::ToursRepository;

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
}
