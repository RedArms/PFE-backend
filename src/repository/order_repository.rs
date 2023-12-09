use crate::models::order::Order;
use sqlx_core::types::chrono::NaiveDate;
use actix_web::web;
use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(Clone)]
pub struct OrderRepository {
    app_state: web::Data<crate::AppState>,
}

impl OrderRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_orders_from_date_and_tour(
        &self,
        date: String,
        tour: i32,
    ) -> Result<Vec<Order>, Error> {
        let date_parsed = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let orders = sqlx::query_as!(
            Order,
            "SELECT order_id,client,tour,date,CAST(status AS TEXT) as status FROM pfe.orders WHERE date = $1 AND tour = $2",
            date_parsed,
            tour
        )
        .fetch_all(&self.app_state.db_pool)
        .await?;

        Ok(orders)
    }
}
