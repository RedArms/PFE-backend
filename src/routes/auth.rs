use crate::{models::user::User, service::user_service::UserService};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let auth_route = web::scope("/auth")
        .service(login_user)
        .service(register_user);

    cfg.service(auth_route);
}

/*
    Login route
    POST /login
    JSON body: {
        "email": "email",
        "password": "password"
    }

    Return:
        - 200 OK if login successful
        - 401 Unauthorized if login failed
*/
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

/*
    Register route
    POST /register
    JSON body: {
        "last_name": "last_name",
        "first_name": "first_name",
        "email": "email",
        "phone": "phone",
        "password": "password"
    }

    Return:
        - 201 Created if user created
        - 409 Conflict if user already exists (email already used)
*/
#[post("/register")]
async fn register_user(
    user: web::Json<User>,
    user_service: web::Data<UserService>,
) -> HttpResponse {
    let user = user.into_inner();

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
