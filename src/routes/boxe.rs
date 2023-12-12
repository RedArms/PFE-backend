use crate::service::boxe_service::BoxeService;
use actix_web::{error, get, web, HttpResponse, Result};
use crate::models::boxe::Boxe_DTO;
use crate::service::item_service::ItemService;

#[get("/allBoxes/{id}")]
pub async fn get_all_boxes(
    boxe_service: web::Data<BoxeService>,
    item_service: web::Data<ItemService>,
    id: web::Path<i32>,
) -> Result<HttpResponse, error::Error> {
    let boxes = boxe_service.get_all_boxes(id.into_inner()).await.unwrap();
    let mut result: Vec<Boxe_DTO> = Vec::new();
    for boxe in &boxes {
        let item_info = item_service.get_item(boxe.item).await.unwrap().unwrap();
        result.push(Boxe_DTO {
            order_id: boxe.order_id,
            name : item_info.label,
            size: item_info.size,
            delivered_qty: boxe.delivered_qty,
            quantity: boxe.quantity,
            box_status: boxe.box_status.clone(),
        });
    }
    match boxes.len() {
        0 => Err(error::ErrorNotFound("No boxes found")),
        _ => Ok(HttpResponse::Ok().json(result)),
    }
}
