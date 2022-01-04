use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").expect("URL is not found");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(app_host)?
    .run()
    .await
}
