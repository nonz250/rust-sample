use actix_web::{HttpResponse, Responder};
use crate::domain::post::post_title::PostTitle;
use crate::domain::post::post_factory::PostFactory;
use crate::domain::post::post_factory::PostMySqlFactory;
use crate::domain::post::post_repoditory::PostMySqlRepository;
use crate::domain::post::post_repoditory::PostRepository;

pub async fn index() -> impl Responder {
    let mut post = PostMySqlFactory::new_post();
    post.change_title(PostTitle { title: "hoge".to_string() });
    PostMySqlRepository::save(post);
    HttpResponse::Ok().body("Data Stored.")
}