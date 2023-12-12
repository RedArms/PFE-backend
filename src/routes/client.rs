use actix_web::{get, post, delete, put, HttpResponse, Result, web, error};
use serde::de;
use crate::service::client_service::ClientService;
use crate::models::regular_order::RegularOrder;
use crate::models::client::Client;

#[get("/")]
async fn get_all_clients(client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.get_all_clients().await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("/")]
async fn add_client(client: web::Json<Client>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.add_client(client.into_inner()).await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[put("/{id}")]
async fn update_client(path: web::Path<i32>, client: web::Json<Client>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.update_client(id, client.into_inner()).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[delete("/{id}")]
async fn delete_client(path: web::Path<i32>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.delete_client(id).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/orders/{id}")]
async fn get_order(path: web::Path<i32>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let regular_order = client_service.get_order(id).await;

    match regular_order {
        Ok(regular_order) => Ok(HttpResponse::Ok().json(regular_order)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[put("/orders/{id}")]
async fn update_order(path: web::Path<i32>, regular_order: web::Json<RegularOrder>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.update_order(id, regular_order.into_inner()).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("/getAllBoxes/{id}/{tourDay}/{date}")]
pub async fn get_all_boxes_client_tour(
        client_service: web::Data<ClientService>,
        path: web::Path<(i32, i32, String)>,
    ) -> Result<HttpResponse, error::Error> {
    let (id, tour_day, date) = path.into_inner();
    let result = client_service.get_all_boxes_client(id, tour_day, date).await.unwrap();

    match result.len() {
        0 => return Err(error::ErrorNotFound("No boxes found")),
        _ => Ok(HttpResponse::Ok().json(result))
        ,
    }
}

