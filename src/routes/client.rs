use crate::service::client_service::ClientService;
use actix_web::{error, get, web, HttpResponse, Result};

#[get("/getAllBoxes/{id}/{tourDay}/{date}")]
pub async fn get_all_boxes_client_tour(
    client_service: web::Data<ClientService>,
    path: web::Path<(i32, i32, String)>,
) -> Result<HttpResponse, error::Error> {
    let (id, tour_day, date) = path.into_inner();
    let result = client_service
        .get_all_boxes(id, tour_day, date)
        .await
        .unwrap();

    match result.len() {
        0 => return Err(error::ErrorNotFound("No boxes found")),
        _ => Ok(HttpResponse::Ok().json(result))
        ,
    }
}