use crate::models::boxe::Boxe;
use crate::routes::boxe::Boxe_Update_DTO;
use actix_web::web;
use sqlx::Error;

#[derive(Clone)]
pub struct BoxeRepository {
    app_state: web::Data<crate::AppState>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "pfe.box_status_type")]
pub enum BoxStatus {
    livre,
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

    pub async fn update_box(&self, id: i32, boxe: Vec<Boxe_Update_DTO>) -> Result<(), Error> {
        for b in boxe {
            println!("Before query");

            // Use match to handle the result of the query execution
            match sqlx::query!(
                "UPDATE pfe.boxes SET delivered_qty = $1, box_status = 'livre'::pfe.box_status_type WHERE order_id = $2 AND item = $3 AND box_status <> 'livre'",
                b.delivered_qty,
                id,
                b.item
            )
                    .execute(&self.app_state.db_pool)
                    .await
                {
                    Ok(_) => {
                        // Handle success
                        println!("Update successful");
                    }
                    Err(err) => {
                        // Handle error
                        eprintln!("Error updating: {:?}", err);
                    }
                }

            println!("After query");
        }

        println!("Finished processing boxes");
        Ok(())
    }
}
