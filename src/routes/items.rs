use crate::{models::item::Item, service::item_service::ItemService};
use actix_web::{error, get, post, web, HttpResponse, Responder, Result};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let item_route = actix_web::web::scope("/items")
        .service(get_item)
        .service(get_items)
        .service(create_item);

    cfg.service(item_route);
}

/**
 * Get an item by id
 * returns the item and status 200 if found or status 404 if not found
 */
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

/**
 * Get all items
 * returns all items and status 200
 */
#[get("/")]
async fn get_items(item_service: web::Data<ItemService>) -> impl Responder {
    let items = item_service.get_all_items().await;

    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/**
 * Create a new item
 * returns the created item and status 201
 */
#[post("/")]
async fn create_item(
    item: web::Json<Item>,
    item_service: web::Data<ItemService>,
) -> Result<HttpResponse, error::Error> {
    let item = item.into_inner();

    let item_result = item_service.create_item(item).await;

    match item_result {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}
