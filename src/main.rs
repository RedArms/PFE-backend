mod routes;
mod models;
mod tests;
mod service;
mod repository;

use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use sqlx::{postgres::PgPool, Error};
use std::env;

// Import functions for each route
use routes::items::get_item;
use routes::users::{get_user, verify_user, revoke_user, set_admin};
use routes::index::{hello, helloworld};
use routes::client::{get_all_clients, add_client, delete_client, get_order, update_order};
use crate::repository::item_repository::ItemRepository;
use crate::repository::user_repository::UserRepository;
use crate::repository::client_repository::ClientRepository;
use crate::service::item_service::ItemService;
use crate::service::user_service::UserService;
use crate::service::client_service::ClientService;

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
    let client_repo = ClientRepository::new(web::Data::new(app_state.clone()));
    let client_service = ClientService::new(client_repo);

    // Start the Actix server
    HttpServer::new(move || {
        let user_route = actix_web::web::scope("/users")
            .service(get_user)
            .service(verify_user)
            .service(revoke_user)
            .service(set_admin);
        let item_route = actix_web::web::scope("/items")
            .service(get_item);
        let client_route = actix_web::web::scope("/client")
            .service(get_all_clients)
            .service(add_client)
            .service(delete_client)
            .service(get_order)
            .service(update_order);
        //index in last because empty route path
        let index_route = actix_web::web::scope("")
            .service(helloworld)
            .service(hello);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone()))// Add ItemService to application data
            .app_data(web::Data::new(client_service.clone()))
            .service(item_route)
            .service(user_route)
            .service(client_route)
            .service(index_route)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
