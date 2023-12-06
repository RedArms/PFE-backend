use actix_web::{get, HttpResponse, Result, web, error};
use crate::repository::{itemRepo, userRepo};

#[get("/{id}")]
async fn get_user(path: web::Path<i32>) ->  Result<HttpResponse,error::Error> {
    let id = path.into_inner();

    let user = userRepo::get_user(id);

    match user.await {
        // Directly match against the result of 'await'
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        // Directly match against the result of 'await'
        None => Err(error::ErrorNotFound("User not found"))

    }

}
