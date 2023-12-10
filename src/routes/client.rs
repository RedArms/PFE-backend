use actix_web::{get, post, delete, put, HttpResponse, Result, web, error, Responder};
use serde::de;
use crate::{service::client_service::ClientService, models::{regular_order, regular_order_line}};


#[get("/")]
async fn get_all_clients(client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.get_all_clients().await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("/")]
async fn add_client(client: web::Json<crate::models::client::Client>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let client_result = client_service.add_client(client.into_inner()).await;

    match client_result {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
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
async fn update_order(path: web::Path<i32>, regular_order: web::Json<regular_order::RegularOrder>, client_service: web::Data<ClientService>) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();

    let result = client_service.update_order(id, regular_order.into_inner()).await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}