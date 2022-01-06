use actix_web::{App, HttpServer};
use sample::routes;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").expect("URL is not found");

    HttpServer::new(|| {
        App::new().configure(routes::routes)
    })
    .bind(app_host)?
    .run()
    .await
}
