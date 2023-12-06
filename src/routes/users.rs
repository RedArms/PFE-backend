use actix_web::{get, HttpResponse, Result, web, error};
use crate::service::userService::UserService;

#[get("/{id}")]
async fn get_user(path: web::Path<i32>, user_service: web::Data<UserService>) ->  Result<HttpResponse,error::Error> {
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
