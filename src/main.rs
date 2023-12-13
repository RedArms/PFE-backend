mod configuration;
mod models;
mod repository;
mod routes;
mod service;
mod tests;

use crate::configuration::route_configuration::configure_routes;
use crate::configuration::injection::configure_app;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chrono::{Datelike};
use dotenv::dotenv;
use sqlx::{postgres::PgPool, Error};

use std::env;
use tokio::task;

use configuration::BD_conf::init_db_pool;
use configuration::creation_tours_automation::run_daily_automation;


#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Create the connection pool
    let db_pool = init_db_pool()
        .await
        .expect("Failed to create database pool");
    // Clone db_pool for the Actix server
    let db_pool_for_server = db_pool.clone();

    task::spawn(async move {
        // Clone db_pool for the asynchronous task
        let db_pool_for_task = db_pool.clone();

        run_daily_automation(AppState {
            db_pool: db_pool_for_task,
        })
            .await;
    });

    let port = env::var("PORT")
        .expect("PORT not found in .env")
        .parse()
        .unwrap();

    // Create the AppState
    let app_state = AppState {
        db_pool: db_pool_for_server.clone(),
    };

    // Print a message to show that the server has started successfully with time
    println!(
        "{} Server is running on port {}",
        chrono::Local::now(),
        port
    );


    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .app_data(web::Data::new(app_state.clone()))
            .configure(|app| configure_app(app, &app_state))  // Use the configure_app function
            .configure(configure_routes)
    })
    //4125 idk why but 8080 dont work
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
