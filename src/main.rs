mod api;
mod models;
mod repository;
mod tests;

use dotenv::dotenv;

use api::notes::{
    hello,
    get_item,
};
use repository::dbconnection_repo::ItemRepository;
use actix_web::{web, App, HttpServer};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Create the repository
    let item_repo = ItemRepository::new(&database_url).await.expect("Failed to create ItemRepository");

    // Start the Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(item_repo.clone()))
            .service(hello)
            .service(get_item)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
