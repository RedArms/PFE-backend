use std::process::id;

use actix_web::{get, HttpResponse, Result, web, error,Responder};
use serde::{Serialize, Deserialize};

use crate::{models::item::Item, repository::test_repo::ItemRepository};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/item/{id}")]
async fn get_item(path: web::Path<i32>,repo:web::Data<ItemRepository>) ->  Result<HttpResponse,error::Error> { 
    let id = path.into_inner();
 
    let item= repo.get_item(id as i32).await.unwrap();
    match item {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        None => Err(error::ErrorNotFound("Item not found")), // Use Actix's ErrorNotFound
    }

}
