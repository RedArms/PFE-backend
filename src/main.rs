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
use routes::index::{hello, helloworld};
use crate::models::user::User;
use crate::repository::itemRepo::ItemRepo;
use crate::repository::userRepo::UserRepo;
use crate::service::itemService::ItemService;
use crate::service::userService::UserService;

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


    let item_repo = ItemRepo::new(web::Data::new(app_state.clone()));
    let item_service = ItemService::new(item_repo);
    let user_repo = UserRepo::new(web::Data::new(app_state.clone()));
    let user_service = UserService::new(user_repo);

    // Start the Actix server
    HttpServer::new(move || {
        let user_route = actix_web::web::scope("/users")
            .service(get_user);
        let item_route = actix_web::web::scope("/items")
            .service(get_item);
        //index in last because empty route path
        let index_route = actix_web::web::scope("/")
            .service(helloworld)
            .service(hello);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone()))// Add ItemService to application data
            .service(item_route)
            .service(user_route)
            .service(index_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
