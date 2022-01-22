use actix_web::{web, HttpResponse, Responder};
use ulid::Ulid;
use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;
use crate::domain::post::post_repoditory::PostMySqlRepository;
use crate::domain::post::post_repoditory::PostRepository;


pub async fn index(web::Path(id): web::Path<String>) -> impl Responder {
    let identifier = PostIdentifier {
        identifier: Ulid::from_string(&id.to_string()).unwrap()
    };
    let mut post = PostMySqlRepository::find_by_id(identifier);
    post.change_title(PostTitle { title: "update".to_string() });
    PostMySqlRepository::save(post);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Data Updated. id: {}", id))
}