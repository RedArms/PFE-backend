use crate::models::tours::Tours;
use sqlx::postgres::PgPool;
use sqlx::Error;
use actix_web::web;

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
}
