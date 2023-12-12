mod models;
mod repository;
mod routes;
mod service;
mod tests;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use chrono::{Datelike, Duration, Local, Utc};
use dotenv::dotenv;
use repository::tours_repository::ToursRepository;
use service::tours_service::ToursService;
use sqlx::{postgres::PgPool, Error};
use sqlx::error::Error as SqlxError;

use std::env;
use std::thread;
use std::time::Duration as StdDuration;

// Import functions for each route
use routes::auth::{login_user, register_user};
use routes::boxe::get_all_boxes;
use routes::index::{hello, helloworld};
use routes::clients::{get_all_clients, add_client, update_client, delete_client, get_order, update_order};
use routes::items::{get_item, get_items, create_item};
use routes::tours::{
    get_all_client_by_tour, get_all_not_delivered, get_all_tours, get_all_tours_day,
    get_tour_by_id, get_tours_by_delivery_day, get_tours_date, get_tours_deliverer_day,
    get_tours_today, set_deliverer,
};
use routes::users::{get_all_users, get_user, revoke_user, set_admin, verify_user};

use crate::repository::boxe_repository::BoxeRepository;
use crate::repository::client_repository::ClientRepository;
use crate::repository::item_repository::ItemRepository;
use crate::repository::order_repository::OrderRepository;
use crate::repository::user_repository::UserRepository;
use crate::routes::clients::get_all_boxes_client_tour;
use crate::service::boxe_service::BoxeService;
use crate::service::client_service::ClientService;
use crate::service::item_service::ItemService;
use crate::service::order_service::OrderService;
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

async fn create_tour_day(db_pool: &PgPool) -> Result<(), sqlx::Error > {
    // Get the current date 

    println!("Creating tour day");
    let current_date = Utc::now();
    let new_date = current_date + Duration::days(3);
    if new_date.weekday().number_from_monday() >5 {
        return Err(SqlxError::Io(*Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Weekend: no tour day created"))));
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
            thread::sleep(StdDuration::from_secs(duration_until_next_day.num_seconds() as u64));

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
        let user_route = actix_web::web::scope("/users")
            .service(get_user)
            .service(get_all_users)
            .service(verify_user)
            .service(revoke_user)
            .service(set_admin);

        let client_route = actix_web::web::scope("/clients")
            .service(get_all_clients)
            .service(add_client)
            .service(update_client)
            .service(delete_client)
            .service(get_order)
            .service(update_order)
            .service(get_all_boxes_client_tour);
        let item_route = actix_web::web::scope("/items")
            .service(get_item)
            .service(get_items)
            .service(create_item);
        let tour_route = actix_web::web::scope("/tours")
            .service(get_all_tours_day)
            .service(get_tours_date)
            .service(get_tours_by_delivery_day)
            .service(get_all_tours)
            .service(get_tours_today)
            .service(get_all_not_delivered)
            .service(get_tours_deliverer_day)
            .service(set_deliverer)
            .service(get_tour_by_id)
            .service(get_all_client_by_tour);
        let boxe_route = actix_web::web::scope("/boxes")
            .service(get_all_boxes)
            .service(get_tours_deliverer_day);
        let auth_route = actix_web::web::scope("/auth")
            .service(login_user)
            .service(register_user);

        //index in last because empty route path
        let index_route = actix_web::web::scope("").service(helloworld).service(hello);

        //test all the workflow

        App::new()
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(user_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(order_service.clone()))
            .app_data(web::Data::new(tours_service.clone()))
            .app_data(web::Data::new(boxe_service.clone())) // Add ItemService to application data
            .app_data(web::Data::new(client_service.clone()))
            .service(item_route)
            .service(user_route)
            .service(auth_route)
            .service(tour_route)
            .service(boxe_route)
            .service(client_route)
            .service(index_route)
    })
    //4125 idk why but 8080 dont work
    .bind(("0.0.0.0", port))?
    .run()
    .await
}