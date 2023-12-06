use actix_web::{post, HttpResponse, Result, web, error};
use serde::{Serialize, Deserialize};
use crate::service::user_service::UserService;


#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
async fn login_user(
    login_info: web::Json<LoginRequest>, user_service: web::Data<UserService>) ->  Result<HttpResponse , error::Error> {
    let email = &login_info.email;
    let password = &login_info.password;
    println!("email: {} password : {}", email , password);
    
   match user_service.login(email, password).await {
       Ok(user) => Ok(HttpResponse::Ok().json(user)),
       Err(e) => Err(error::ErrorBadRequest(e.to_string()))
   }

}