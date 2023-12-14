use crate::models::order::Order;
use actix_web::web;
use sqlx::Error;

#[derive(Clone)]
pub struct OrderRepository {
    app_state: web::Data<crate::AppState>,
}

impl OrderRepository {
    pub fn new(app_state: web::Data<crate::AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Order>, Error> {
        let order = sqlx::query_as!(
            Order,
            "SELECT order_id,client,tour,date,CAST(status AS TEXT) as status FROM pfe.orders WHERE order_id = $1",
            id
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(order)
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
    pub async fn get_order_id(&self, client: i32, tour: i32, date: String) -> Result<i32, Error> {
        let date_parsed = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let order = sqlx::query!(
            "SELECT order_id FROM pfe.orders WHERE client = $1 AND tour = $2 AND date = $3",
            client,
            tour,
            date_parsed
        )
        .fetch_optional(&self.app_state.db_pool)
        .await?;

        Ok(order.map(|o| o.order_id).unwrap_or(0))
    }

    pub async fn set_state_delivering(&self, date: String, tour: i32) -> Result<u64, Error> {
        let date_parsed = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let orders = sqlx::query!(
            "UPDATE pfe.orders SET status = 'en cours de livraison' WHERE date = $1 AND tour = $2",
            date_parsed,
            tour
        )
        .execute(&self.app_state.db_pool)
        .await?;

        Ok(orders.rows_affected())
    }

    pub async fn set_delivered(&self, id: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE pfe.orders SET status = 'livre' WHERE order_id = $1",
            id
        )
        .execute(&self.app_state.db_pool)
        .await?;

        Ok(())
    }
}
