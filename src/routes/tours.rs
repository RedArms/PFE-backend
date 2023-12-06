use crate::service::tours_repository::ToursService;
use actix_web::{error, get, web, HttpResponse, Result};

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
