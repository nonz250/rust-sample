use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use crate::domain::error::error_response::ErrorResponse;
use crate::domain::post::post_title::PostTitle;
use crate::domain::post::command::create_post::CreatePost;
use crate::domain::post::command::create_post::CreatePostInput;

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    title: String
}

pub async fn index(request: web::Json<CreatePostRequest>) -> impl Responder {
    let post_title = match PostTitle::new(request.title.to_string()) {
        Ok(post_title) => post_title,
        Err(err) => return HttpResponse::BadRequest()
            .json(ErrorResponse {
                message: err.to_string()
            })
    };

    CreatePost::process(CreatePostInput {
        title: post_title
    });

    HttpResponse::NoContent().finish()
}