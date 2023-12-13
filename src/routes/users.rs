use crate::service::user_service::UserService;
use actix_web::{delete, error, get, post, web, HttpResponse, Result};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let user_route = actix_web::web::scope("/users")
        .service(get_user)
        .service(get_all_users)
        .service(verify_user)
        .service(revoke_user)
        .service(set_admin);

    cfg.service(user_route);
}

#[get("/{id}")]
async fn get_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let user = user_service.get_user(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("Item not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[get("")]
async fn get_all_users(user_service: web::Data<UserService>) -> Result<HttpResponse, error::Error> {
    let users = user_service.get_all_users().await;
    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("/verify/{id}")]
async fn verify_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let user = user_service.verify_user(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[delete("/revoke/{id}")]
async fn revoke_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let user = user_service.revoke_user(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("setadmin/{id}")]
async fn set_admin(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    let user = user_service.set_admin(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}
