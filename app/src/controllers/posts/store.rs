use actix_web::{HttpResponse, Responder};
use diesel::RunQueryDsl;
use crate::models::NewPosts;
use crate::schema::posts;
use crate::utils::establish_connection;

pub async fn index() -> impl Responder {
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