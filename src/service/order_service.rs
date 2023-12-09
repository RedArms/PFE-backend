use crate::models::order::Order;
use crate::repository::order_repository::OrderRepository;
use sqlx::Error;

#[derive(Clone)]
pub struct OrderService {
    order_repo: OrderRepository,
}

impl OrderService {
    pub fn new(order_repo: OrderRepository) -> Self {
        Self { order_repo }
    }

    pub async fn get_orders_from_date_and_tour(
        &self,
        date: String,
        tour: i32,
    ) -> Result<Vec<Order>, Error> {
        self.order_repo
            .get_orders_from_date_and_tour(date, tour)
            .await
    }
    

}
