use actix_web::{get, web, HttpResponse, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let index_route = web::scope("").service(helloworld).service(hello);
    cfg.service(index_route);
}

#[get("/")]
async fn helloworld() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();

    HttpResponse::Ok().body(format!("Hello {name}"))
}
