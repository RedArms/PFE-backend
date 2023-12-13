mod configuration;
mod models;
mod repository;
mod routes;
mod service;
mod tests;

use crate::configuration::route_configuration::configure_routes;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chrono::{Datelike, Duration, Local, Utc};
use dotenv::dotenv;
use repository::tours_repository::ToursRepository;
use service::tours_service::ToursService;
use sqlx::error::Error as SqlxError;
use sqlx::{postgres::PgPool, Error};

use std::env;
use std::thread;
use std::time::Duration as StdDuration;
use configuration::BD_conf::init_db_pool;

use crate::repository::boxe_repository::BoxeRepository;
use crate::repository::client_repository::ClientRepository;
use crate::repository::item_repository::ItemRepository;
use crate::repository::order_repository::OrderRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::boxe_service::BoxeService;
use crate::service::client_service::ClientService;
use crate::service::item_service::ItemService;
use crate::service::order_service::OrderService;
use crate::service::user_service::UserService;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}



async fn create_tour_day(db_pool: &PgPool) -> Result<(), sqlx::Error> {
    // Get the current date

    println!("Creating tour day");
    let current_date = Utc::now();
    let new_date = current_date + Duration::days(3);
    if new_date.weekday().number_from_monday() > 5 {
        return Err(SqlxError::Io(*Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Weekend: no tour day created",
        ))));
    }
    let formatted_date =
        chrono::NaiveDate::parse_from_str(&new_date.format("%Y-%m-%d").to_string(), "%Y-%m-%d")
            .unwrap();
    sqlx::query!("call create_tour_day( $1 );", formatted_date)
        .execute(db_pool)
        .await?;

    Ok(())
}

async fn run_daily_automation(app_state: AppState) {
    // Démarrez une tâche quotidienne dans un thread séparé
    tokio::spawn(async move {
        // Clone the database pool from the app_state
        let db_pool = app_state.db_pool.clone();

        loop {
            let now = Local::now();

            let next_day = now.date().succ_opt().unwrap_or(now.date()).and_hms(0, 0, 0);

            let duration_until_next_day = next_day.signed_duration_since(now);

            let create_run = create_tour_day(&db_pool).await;
            thread::sleep(StdDuration::from_secs(
                duration_until_next_day.num_seconds() as u64,
            ));

            // Pass db_pool as an argument to create_tour_day
            match create_run {
                Ok(()) => println!("Tuple créé avec succès"),
                Err(e) => eprintln!("Erreur lors de la création du tuple : {:?}", e),
            }
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Create the connection pool
    let db_pool = init_db_pool()
        .await
        .expect("Failed to create database pool");

    run_daily_automation(AppState {
        db_pool: db_pool.clone(),
    })
    .await;

    let port = env::var("PORT")
        .expect("PORT not found in .env")
        .parse()
        .unwrap();

    // Create the AppState
    let app_state = AppState {
        db_pool: db_pool.clone(),
    };

    // Print a message to show that the server has started successfully with time
    println!(
        "{} Server is running on port {}",
        chrono::Local::now(),
        port
    );

    let item_repo = ItemRepository::new(web::Data::new(app_state.clone()));
    let item_service = ItemService::new(item_repo.clone());
    let user_repo = UserRepository::new(web::Data::new(app_state.clone()));
    let user_service = UserService::new(user_repo.clone());
    let order_repo = OrderRepository::new(web::Data::new(app_state.clone()));
    let order_service = OrderService::new(order_repo.clone());
    let tours_repo = ToursRepository::new(web::Data::new(app_state.clone()));
    let tours_service = ToursService::new(tours_repo.clone(), order_repo.clone());
    let boxe_repo = BoxeRepository::new(web::Data::new(app_state.clone()));
    let boxe_service = BoxeService::new(boxe_repo.clone());
    let client_repo = ClientRepository::new(web::Data::new(app_state.clone()));
    let client_service =
        ClientService::new(client_repo.clone(), boxe_repo.clone(), order_repo.clone());
    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(order_service.clone()))
            .app_data(web::Data::new(tours_service.clone()))
            .app_data(web::Data::new(boxe_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(client_service.clone()))
            .configure(configure_routes)
    })
    //4125 idk why but 8080 dont work
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
