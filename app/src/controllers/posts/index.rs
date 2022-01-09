use actix_web::{HttpResponse, Responder};
use crate::domain::post::query::get_post::GetPostMockQuery;
use crate::domain::post::query::get_post::GetPostMySqlQuery;
use crate::domain::post::query::get_post::GetPostQuery;

pub async fn index() -> impl Responder {
    let post_mysql = GetPostMySqlQuery::get_post();
    let post_mock = GetPostMockQuery::get_post();
    HttpResponse::Ok().body(
        format!(
            "{}:{}:{}:{}:{}",
            "top".to_string(),
            post_mysql.identifier.to_string(),
            post_mysql.title.title,
            post_mock.identifier.to_string(),
            post_mock.title.title
        )
    )
}
