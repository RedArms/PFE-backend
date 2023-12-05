use actix_web::{get, HttpResponse, Result, web, error,Responder};

#[get("/")]
async fn helloworld() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();

    HttpResponse::Ok().body(format!("Hello {name}"))
}