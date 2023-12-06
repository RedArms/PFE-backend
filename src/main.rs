mod models;
mod repository;
mod routes;
mod service;
mod tests;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use repository::tours_repository::ToursRepository;
use service::tours_repository::ToursService;
use sqlx::{postgres::PgPool, Error};
use std::env;

// Import functions for each route
use crate::repository::item_repository::ItemRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::item_service::ItemService;
use crate::service::user_service::UserService;
use routes::index::{hello, helloworld};
use routes::items::get_item;
use routes::tours::get_all_tours;
use routes::users::get_user;

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
    let db_pool = init_db_pool()
        .await
        .expect("Failed to create database pool");

    // Create the AppState
    let app_state = AppState {
        db_pool: db_pool.clone(),
    };

    let item_repo = ItemRepository::new(web::Data::new(app_state.clone()));
    let item_service = ItemService::new(item_repo);
    let user_repo = UserRepository::new(web::Data::new(app_state.clone()));
    let user_service = UserService::new(user_repo);
    let tours_repo = ToursRepository::new(web::Data::new(app_state.clone()));
    let tours_service = ToursService::new(tours_repo);
    // Start the Actix server
    HttpServer::new(move || {
        let user_route = actix_web::web::scope("/users").service(get_user);
        let item_route = actix_web::web::scope("/items").service(get_item);
        let tours_route = actix_web::web::scope("/tours").service(get_all_tours);

        //index in last because empty route path
        let index_route = actix_web::web::scope("").service(helloworld).service(hello);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(tours_service.clone())) // Add ItemService to application data
            .service(item_route)
            .service(user_route)
            .service(tours_route)
            .service(index_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
