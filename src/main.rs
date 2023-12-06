mod routes;
mod models;
mod tests;

mod ucc;

use dotenv::dotenv;
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

    // Create the repository
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

        //App 

        App::new()
            .service(item_route)
            .service(user_route)
            .service(index_route) //index in last because empty route path
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
