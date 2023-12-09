use crate::models::boxe::Boxe;
use crate::repository::boxe_repository::BoxeRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct BoxeService {
    boxe_repo: BoxeRepository,
}

impl BoxeService {
    pub fn new(boxe_repo: BoxeRepository) -> Self {
        Self { boxe_repo }
    }

    pub async fn get_all_boxes(&self, id: i32) -> Result<Vec<Boxe>, Error> {
        self.boxe_repo.get_all_boxes(id).await
    }
}
