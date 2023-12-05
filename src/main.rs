mod routes;
mod models;
mod tests;

use dotenv::dotenv;
use models::pgsqlConn::pgsqlConn;
use actix_web::{web, App, HttpServer};

//Import func of each routes
use routes::items::get_item;

use routes::users::get_user;

use routes::index::{
    hello, helloworld
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Create the repository
    let item_repo = pgsqlConn::new(&database_url).await.expect("Failed to create ItemRepository");

    // Start the Actix server
    HttpServer::new(move || {


        let user_route = actix_web::web::scope("/user")
        .service(get_user);

        let item_route = actix_web::web::scope("/item")
        .service(get_item);

        //index in last because empty route path
        let index_route = actix_web::web::scope("")
        .service(helloworld)
        .service(hello);


        App::new()
            .app_data(web::Data::new(item_repo.clone()))
            .service(item_route)
            .service(user_route)
            .service(index_route) //index in last because empty route path
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
