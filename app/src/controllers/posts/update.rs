use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;
use ulid::Ulid;
use crate::domain::error::error_response::ErrorResponse;
use crate::domain::post::command::update_post::UpdatePost;
use crate::domain::post::command::update_post::UpdatePostInput;
use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;

#[derive(Serialize, Deserialize)]
pub struct UpdatePostRequest {
    title: String
}

pub async fn index(web::Path(id): web::Path<String>, request: web::Json<UpdatePostRequest>) -> impl Responder {
    let identifier = PostIdentifier {
        identifier: match Ulid::from_string(&id.to_string()) {
            Ok(identifier) => identifier,
            Err(_err) => return HttpResponse::BadRequest().json(ErrorResponse {
                message: "Invalid id.".to_string()
            })
        }
    };

    let post_title = match PostTitle::new(request.title.to_string()) {
        Ok(post_title) => post_title,
        Err(err) => return HttpResponse::BadRequest().json(ErrorResponse {
                message: err.to_string()
            })
    };

    match UpdatePost::process(UpdatePostInput {
        identifier: identifier,
        title: post_title
    }) {
        Ok(result) => result,
        Err(err) => return HttpResponse::NotFound().json(ErrorResponse {
                message: err.to_string()
            })
    };

    HttpResponse::NoContent().finish()
}