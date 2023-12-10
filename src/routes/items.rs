use crate::{service::item_service::ItemService,models::item::Item};
use actix_web::{error, get, post, web, HttpResponse, Responder, Result};


#[get("/{id}")]
async fn get_item(
    path: web::Path<i32>,
    item_service: web::Data<ItemService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let item_result = item_service.get_item(id).await;

    match item_result {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Err(error::ErrorNotFound("Item not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/")]
async fn get_items(item_service: web::Data<ItemService>) -> impl Responder {
    let items = item_service.get_all_items().await;

    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/")]
async fn create_item(
    item: web::Json<Item>,
    item_service: web::Data<ItemService>,
) -> Result<HttpResponse, error::Error> {
    let item = item.into_inner();

    let item_result = item_service.create_item(item).await;

    match item_result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}
