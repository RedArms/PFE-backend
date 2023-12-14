use crate::{
    models::order::Order,
    service::{
      order_service::OrderService
    },
};
use actix_web::{error, get,web, HttpResponse, Result};


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let order_route = web::scope("/orders")
        .service(get_order);

    cfg.service(order_route);
}


#[get("/{id}")]
async fn get_order(
    order_service:  web::Data<OrderService>,
    path: web::Path<i32>,
) -> Result<HttpResponse, error::Error> {
    println!("test");
    let id = path.into_inner();
    let order = order_service.get_by_id(id).await;
    match order {
        Ok(None) => Err(error::ErrorNotFound("order not found")),
        Ok(order) => Ok(HttpResponse::Ok().json(order)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

