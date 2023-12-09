use crate::service::{tours_service::ToursService, order_service::{OrderService, self}};
use actix_web::{error, get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn get_all_tours(
    tours_service: web::Data<ToursService>,
) -> Result<HttpResponse, error::Error> {
    let tours = tours_service.get_all().await;
    match tours {
        Ok(tours) => Ok(HttpResponse::Ok().json(tours)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/toursToday")]
async fn get_tours_today(
    tours_service: web::Data<ToursService>,
) -> Result<HttpResponse, error::Error> {
    let tours = tours_service.get_tours_today().await;
    match tours {
        Ok(tours) => Ok(HttpResponse::Ok().json(tours)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}



#[get("/{tour}/{date}")]
async fn get_tours_deliverer_day(
    order_service: web::Data<OrderService>,
    path: web::Path<(i32, String)>,
) -> Result<HttpResponse, error::Error> {
    let (tour, date) = path.into_inner(); 
    let tours = order_service.get_orders_from_date_and_tour(date, tour).await;
    match tours {
        Ok(tours) => Ok(HttpResponse::Ok().json(tours)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

