use actix_web::{HttpResponse, Responder};
// Mock query sample.
// use crate::domain::post::query::get_post::GetPostMockQuery;
use crate::domain::post::query::get_post::GetPostMySqlQuery;
use crate::domain::post::query::get_post::GetPostQuery;

pub async fn index() -> impl Responder {
    let posts_mysql = GetPostMySqlQuery::get_post();
    // Mock query sample.
    // let posts_mock = GetPostMockQuery::get_post();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(posts_mysql)
}
