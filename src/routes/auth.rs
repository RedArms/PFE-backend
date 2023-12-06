use crate::{models::user::User, service::user_service::UserService};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
async fn login_user(
    login_info: web::Json<LoginRequest>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    let email = &login_info.email;
    let password = &login_info.password;

    let user_found: Option<User> = user_service.login(email, password).await;

    match user_found {
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

// register login route given user models
#[post("/register")]
async fn register_user(
    user: web::Json<User>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    let user = user.into_inner();

    print!("On est dans le register ou quuoient laa ");

    let user_found = user_service.get_user_by_email(&user.email).await;

    // Check if user already exists
    match user_found {
        Ok(Some(_)) => {
            // User already exists
            HttpResponse::Conflict().json("User already exists")
        }
        Ok(None) => {
            // User does not exist, create it , return user_id if success
            let user_id = user_service.register(user).await;

            match user_id {
                Ok(user_id) => {
                    // User created
                    HttpResponse::Created().json(user_id)
                }
                Err(_) => {
                    // Error occurred
                    HttpResponse::InternalServerError().json("Internal Server Error")
                }
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}
