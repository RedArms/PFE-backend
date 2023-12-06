use actix_web::{post, HttpResponse, web};
use serde::{Serialize, Deserialize};
use crate::{service::user_service::UserService, models::user::User};


#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
async fn login_user(
    login_info: web::Json<LoginRequest>, user_service: web::Data<UserService>) -> HttpResponse {
    let email = &login_info.email;
    let password = &login_info.password;


    let user_found: Option<User>= user_service.login(email, password).await;

    match user_found{
        Some(user) => {
            // User found, password correct
            HttpResponse::Ok().json(user)
        }
        None => {
            // User not found
            HttpResponse::Unauthorized().json("Mauvais email ou mot de passe")
        }
    }

}