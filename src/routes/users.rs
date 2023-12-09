use actix_web::{get, delete, post, HttpResponse, Result, web, error};
use crate::service::user_service::UserService;

#[get("/{id}")]
async fn get_user(
    path: web::Path<i32>,
    user_service: web::Data<UserService>,
) -> Result<HttpResponse, error::Error> {
    let id = path.into_inner();
    print!("on passe 1 ");
    let user = user_service.get_user(id).await;
    print!("on passe 2");
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("Item not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[post("/verify/{id}")]
async fn verify_user(path: web::Path<i32>, user_service: web::Data<UserService>) ->  Result<HttpResponse,error::Error> {
    let id = path.into_inner();
    let user = user_service.verify_user(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}

#[delete("/revoke/{id}")]  
async fn revoke_user(path: web::Path<i32>, user_service: web::Data<UserService>) ->  Result<HttpResponse,error::Error> {
    let id = path.into_inner();
    let user = user_service.revoke_user(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}


#[post("setadmin/{id}")]
async fn set_admin(path: web::Path<i32>, user_service: web::Data<UserService>) ->  Result<HttpResponse,error::Error> {
    let id = path.into_inner();
    let user = user_service.set_admin(id).await;
    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Err(error::ErrorNotFound("User not found")),
        Err(_) => Err(error::ErrorInternalServerError("Internal Server Error")),
    }
}