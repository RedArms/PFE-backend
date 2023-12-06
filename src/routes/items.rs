use actix_web::{get, HttpResponse, Result, web, error};
use crate::repository::itemRepo;

#[get("/{id}")]
async fn get_item(path: web::Path<i32>) ->  Result<HttpResponse,error::Error> {
    let id = path.into_inner();

    let item = itemRepo::get_item(id);


    match item.await {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        // Directly match against the result of 'await'
        None => Err(error::ErrorNotFound("Item not found"))
    }
}
