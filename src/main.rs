mod models;
mod repository;
mod routes;
mod service;
mod tests;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use repository::tours_repository::ToursRepository;
use service::tours_service::ToursService;
use sqlx::{postgres::PgPool, Error};
use std::env;

// Import functions for each route
use routes::items::get_item;

use crate::repository::boxe_repository::BoxeRepository;
use crate::repository::item_repository::ItemRepository;
use crate::repository::order_repository::OrderRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::boxe_service::BoxeService;
use crate::service::item_service::ItemService;
use crate::service::order_service::OrderService;
use crate::service::user_service::UserService;
use routes::auth::{login_user, register_user};
use routes::boxe::get_all_boxes;
use routes::index::{hello, helloworld};
use routes::tours::{get_all_tours, get_tours_deliverer_day, get_tours_today, set_deliverer};
use routes::users::{get_user, revoke_user, set_admin, verify_user};

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
    let order_repo = OrderRepository::new(web::Data::new(app_state.clone()));
    let order_service = OrderService::new(order_repo);
    let boxe_repo = BoxeRepository::new(web::Data::new(app_state.clone()));
    let boxe_service = BoxeService::new(boxe_repo);
    // Start the Actix server
    HttpServer::new(move || {
        let user_route = actix_web::web::scope("/users")
            .service(get_user)
            .service(verify_user)
            .service(revoke_user)
            .service(set_admin);
        let item_route = actix_web::web::scope("/items").service(get_item);
        let tour_route = actix_web::web::scope("/tours")
            .service(get_all_tours)
            .service(get_tours_today)
            .service(get_tours_deliverer_day)
            .service(set_deliverer);
        let boxe_route = actix_web::web::scope("/boxes").service(get_all_boxes);
        let auth_route = actix_web::web::scope("/auth")
            .service(login_user)
            .service(register_user);

        //index in last because empty route path
        let index_route = actix_web::web::scope("").service(helloworld).service(hello);

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(order_service.clone()))
            .app_data(web::Data::new(tours_service.clone()))
            .app_data(web::Data::new(boxe_service.clone())) // Add ItemService to application data
            .service(item_route)
            .service(user_route)
            .service(auth_route)
            .service(tour_route)
            .service(boxe_route)
            .service(index_route)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
