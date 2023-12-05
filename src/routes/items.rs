use actix_web::{get, HttpResponse, Result, web, error,Responder};

use crate::models::PgsqlConn::pgsql_conn;

#[get("/{id}")]
async fn get_item(path: web::Path<i32>,repo:web::Data<pgsql_conn>) ->  Result<HttpResponse,error::Error> { 
    let id = path.into_inner();
 
    let item= repo.get_item(id as i32).await.unwrap();
    match item {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        None => Err(error::ErrorNotFound("Item not found")), // Use Actix's ErrorNotFound
    }

}