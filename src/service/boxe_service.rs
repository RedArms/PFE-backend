use crate::models::boxe::Boxe;
use crate::repository::boxe_repository::BoxeRepository;
use crate::routes::boxe::Boxe_Update_DTO;
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

    pub async fn update_box(&self, id: i32, boxe: Vec<Boxe_Update_DTO>) -> Result<(), Error> {
        println!("test 212");
        self.boxe_repo.update_box(id, boxe).await
    }
}
