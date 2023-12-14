use crate::models::boxe::{Boxe, Boxe_DTO};
use crate::service::boxe_service::BoxeService;
use crate::service::item_service::ItemService;
use actix_web::{error, get, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::service::order_service::OrderService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let boxe_route = web::scope("/boxes")
        .service(get_all_boxes)
        .service(update_box);
    cfg.service(boxe_route);
}

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
            item_id: boxe.item,
            name: item_info.label,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Boxe_Update_DTO {
    pub order_id: i32,
    pub item: i32,
    pub delivered_qty: i32,
}

#[put("/updateBox/{idOrder}")]
pub async fn update_box(
    boxe_service: web::Data<BoxeService>,
    order_service : web::Data<OrderService>,
    idOrder: web::Path<i32>,
    boxes: web::Json<Vec<Boxe_Update_DTO>>,
) -> Result<HttpResponse, error::Error> {
    let id = idOrder.into_inner();
    let result = boxe_service.update_box(id, boxes.into_inner()).await;
    order_service.set_delivered(id).await.unwrap();

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}
