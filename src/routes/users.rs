use actix_web::{get,post, HttpResponse, Result, web, error};

use crate::models::pgsqlConn::pgsqlConn;
use crate::models::user::User;

#[get("/{id}")]
async fn get_user(path: web::Path<i32>,repo:web::Data<pgsqlConn>) ->  Result<HttpResponse,error::Error> { 
    let id = path.into_inner();
 
    let item= repo.get_item(id as i32).await.unwrap();
    match item {
        Some(item) => Ok(HttpResponse::Ok().json(item)),
        None => Err(error::ErrorNotFound("User not found")), // Use Actix's ErrorNotFound
    }

}

#[post("/new")]
async fn new_user(info: web::Json<User>) -> Result<HttpResponse,error::Error> {
    let user = info.into_inner();
    Ok(HttpResponse::Ok().json(user))
}