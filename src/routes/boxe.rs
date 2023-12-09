use crate::service::boxe_service::BoxeService;
use actix_web::{error, get, web, HttpResponse, Result};

#[get("/allBoxes/{id}")]
pub async fn get_all_boxes(
    boxe_service: web::Data<BoxeService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, error::Error> {
    let boxes = boxe_service.get_all_boxes(id.into_inner()).await.unwrap();
    match boxes.len() {
        0 => Err(error::ErrorNotFound("No boxes found")),
        _ => Ok(HttpResponse::Ok().json(boxes)),
    }
}
