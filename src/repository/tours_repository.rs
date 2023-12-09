use crate::models::tours::Tours;
use crate::models::tours_day::ToursDay;
use actix_web::web;
use sqlx::Error;

#[derive(Clone)]
pub struct ToursRepository {
    app_state: web::Data<crate::AppState>,
}

impl ToursRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_all_tours(&self) -> Result<Vec<Tours>, Error> {
        let tours = sqlx::query_as!(Tours, "SELECT * FROM pfe.tours")
            .fetch_all(&self.app_state.db_pool)
            .await?;

        Ok(tours)
    }

    pub async fn get_tours_deliverer_day(&self, deliverer: i32) -> Result<Tours, Error> {
        let tour = sqlx::query_as!(ToursDay, "SELECT * FROM pfe.tour_day WHERE tour_id = $1", tour_id)
            .fetch_one(&self.app_state.db_pool)
            .await?;

        Ok(tour)
    }
}
