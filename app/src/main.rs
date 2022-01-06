use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::RunQueryDsl;
use sample::models::NewPosts;
use sample::schema::posts;
use sample::utils::establish_connection;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/store")]
async fn store() -> impl Responder {
    let connection = establish_connection();
    let new_post = NewPosts {
        title: String::from("hoge"),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new posts.");

    HttpResponse::Ok().body("Data Stored.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_host = env::var("APP_HOST").expect("URL is not found");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(store)
    })
    .bind(app_host)?
    .run()
    .await
}
