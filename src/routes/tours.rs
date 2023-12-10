use crate::service::{
    order_service::{self, OrderService},
    tours_service::ToursService,
    client_service::ClientService,
};
use actix_web::{error, get, post, web, HttpResponse, Result};
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

#[get("/{id}")]
async fn get_tour_by_id(
    tours_service: web::Data<ToursService>,
    path: web::Path<i32>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let tour = tours_service.get_by_id(id).await;
    match tour {
        Ok(None) => Err(error::ErrorNotFound("Tour not found")),
        Ok(tour) => Ok(HttpResponse::Ok().json(tour)),
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
    let tours = order_service
        .get_orders_from_date_and_tour(date, tour)
        .await;
    match tours {
        Ok(tours) => Ok(HttpResponse::Ok().json(tours)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[derive(Debug, Deserialize)]
pub struct SetDelivererRequest {
    pub tour: i32,
    pub date: String,
    pub delivery_person: i32,
}

#[post("/setDeliverer")]
async fn set_deliverer(
    tours_service: web::Data<ToursService>,
    payload: web::Json<SetDelivererRequest>,
) -> Result<HttpResponse, error::Error> {
    match tours_service
        .set_deliverer(payload.tour, payload.date.clone(), payload.delivery_person)
        .await
    {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                Ok(HttpResponse::Ok().json(""))
            } else {
                Err(error::ErrorNotFound("No rows found or updated"))
            }
        }
        Err(err) => Err(error::ErrorInternalServerError(format!(
            "Internal Server Error: {}",
            err
        ))),
    }
}

#[get("/getAllNotDelivered")]
async fn get_all_not_delivered(
    tour_service: web::Data<ToursService>,
) -> Result<HttpResponse, error::Error> {
    let tours_day = tour_service.get_tours_day_avalaible().await;
    match tours_day {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/getTours/{id}/getAllClient")]
async fn get_all_client_by_tour(
    client_service: web::Data<ClientService>,
    path: web::Path<i32>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let tour = client_service.get_all_client_by_tour(id).await;
    match tour {
        Ok(tour) => Ok(HttpResponse::Ok().json(tour)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}
