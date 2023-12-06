mod routes;
mod models;
mod tests;
mod service;
mod repository;

use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use sqlx::{postgres::PgPool, Error};
use std::env;
use actix_web::web::get;

// Import functions for each route
use routes::items::get_item;
use routes::users::get_user;
use routes::auth::{login_user, register_user};
use routes::index::{hello, helloworld};
use crate::models::user::User;
use crate::repository::item_repository::ItemRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::item_service::ItemService;
use crate::service::user_service::UserService;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

async fn init_db_pool() -> Result<PgPool, Error> {
    dotenv().ok();

    // Retrieve the database URL from the environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env");

    // Connect to the database
    PgPool::connect(&database_url).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Create the connection pool
    let db_pool = init_db_pool().await.expect("Failed to create database pool");

    // Create the AppState
    let app_state = AppState { db_pool: db_pool.clone() };


    let item_repo = ItemRepository::new(web::Data::new(app_state.clone()));
    let item_service = ItemService::new(item_repo);
    let user_repo = UserRepository::new(web::Data::new(app_state.clone()));
    let user_service = UserService::new(user_repo);

    // Start the Actix server
    HttpServer::new(move || {
        let user_route = actix_web::web::scope("/users")
            .service(get_user);
        let item_route = actix_web::web::scope("/items")
            .service(get_item);
        //index in last because empty route path
        let index_route = actix_web::web::scope("")
            .service(helloworld)
            .service(hello);

        let auth_route = actix_web::web::scope("/auth")
            .service(login_user)
            .service(register_user);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone()))// Add ItemService to application data
            .service(item_route)
            .service(user_route)
            .service(auth_route)
            .service(index_route)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
