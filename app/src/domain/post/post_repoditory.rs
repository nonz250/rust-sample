use diesel::prelude::*;
use ulid::Ulid;
use crate::domain::post::post::Post;
use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;
use crate::models::NewPosts;
use crate::schema::posts as posts_schema;
use crate::utils::establish_connection;
use crate::models::Post as PostModel;

pub struct PostMySqlRepository {}

pub trait PostRepository {
    fn find_by_id(id: PostIdentifier) -> Result<Post, String>;
    fn save(post: Post);
}

impl PostRepository for PostMySqlRepository {
    fn find_by_id(id: PostIdentifier) -> Result<Post, String> {
        let connection = establish_connection();

        let post = match posts_schema::dsl::posts
            .find(id.to_string())
            .first::<PostModel>(&connection) {
                Ok(ret) => ret,
                Err(_err) => return Err("Post not found".to_string())
            };

        return Ok(Post::new(
            PostIdentifier { identifier: Ulid::from_string(&post.id).unwrap() },
            PostTitle::new(post.title).unwrap()
        ))
    }

    fn save(post: Post) {
        let connection = establish_connection();
        let new_post = NewPosts {
            id: post.identifier(),
            title: post.title.title(),
        };
        diesel::replace_into(posts_schema::table)
            .values(&new_post)
            .execute(&connection)
            .expect("Error saving new posts.");
    }
}
