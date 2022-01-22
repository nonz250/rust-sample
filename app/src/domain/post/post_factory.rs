use ulid::Ulid;
use crate::domain::post::post::Post;
use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;

pub struct PostMySqlFactory {}

pub trait PostFactory {
    fn new_post() -> Post;
}

impl PostFactory for PostMySqlFactory {
    fn new_post() -> Post {
        return Post::new(
            PostIdentifier { identifier: Ulid::new() },
            PostTitle { title: "".to_string() }
        )
    }
}