use actix_web::{get, HttpResponse, web,Responder};

#[get("/")]
async fn helloworld() -> impl Responder {
    print!("QUOICOUBAKA");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn hello(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();

    HttpResponse::Ok().body(format!("Hello {name}"))
}