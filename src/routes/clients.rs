use crate::models::boxe::Boxe_DTO;
use crate::models::client::Client;
use crate::models::regular_order::RegularOrder;
use crate::service::client_service::ClientService;
use crate::service::item_service::ItemService;
use actix_web::{delete, error, get, post, put, web, HttpResponse, Result};
use serde::de;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let client_route = web::scope("/clients")
        .service(get_all_clients)
        .service(add_client)
        .service(update_client)
        .service(delete_client)
        .service(get_order)
        .service(update_order)
        .service(get_all_boxes_client_tour);

    cfg.service(client_route);
}

#[get("/")]
async fn get_all_clients(
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.get_all_clients().await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("/")]
async fn add_client(
    client: web::Json<Client>,
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.add_client(client.into_inner()).await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[put("/{id}")]
async fn update_client(
    path: web::Path<i32>,
    client: web::Json<Client>,
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.update_client(id, client.into_inner()).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[delete("/{id}")]
async fn delete_client(
    path: web::Path<i32>,
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.delete_client(id).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/orders/{id}")]
async fn get_order(
    path: web::Path<i32>,
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let regular_order = client_service.get_order(id).await;

    match regular_order {
        Ok(regular_order) => Ok(HttpResponse::Ok().json(regular_order)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[put("/orders/{id}")]
async fn update_order(
    path: web::Path<i32>,
    regular_order: web::Json<RegularOrder>,
    client_service: web::Data<ClientService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service
        .update_order(id, regular_order.into_inner())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/getAllBoxes/{id}/{tourDay}/{date}")]
pub async fn get_all_boxes_client_tour(
    client_service: web::Data<ClientService>,
    item_service: web::Data<ItemService>,
    path: web::Path<(i32, i32, String)>,
) -> Result<HttpResponse, error::Error> {
    println!("get_all_boxes_client_tour");
    let (id, tour_day, date) = path.into_inner();
    let boxes = client_service
        .get_all_boxes_client(id, tour_day, date)
        .await
        .unwrap();
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
    match result.len() {
        0 => Err(error::ErrorNotFound("No boxes found")),
        _ => Ok(HttpResponse::Ok().json(result)),
    }
}
