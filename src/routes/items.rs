use crate::service::item_service::ItemService;
use actix_web::{error, get, web, HttpResponse, Result};

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
