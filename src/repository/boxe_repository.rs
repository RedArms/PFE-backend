use crate::models::boxe::Boxe;
use actix_web::web;
use sqlx::Error;

#[derive(Clone)]
pub struct BoxeRepository {
    app_state: web::Data<crate::AppState>,
}

impl BoxeRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }
    
    pub async fn get_all_boxes(&self, id: i32) -> Result<Vec<Boxe>, Error> {
        let boxes = sqlx::query_as!(
            Boxe,
            "SELECT order_id, item, quantity, delivered_qty, CAST(box_status AS TEXT) as box_status FROM pfe.boxes WHERE order_id = $1",
            id
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(boxes)
    }
}
