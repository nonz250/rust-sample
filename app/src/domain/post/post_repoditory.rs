use diesel::prelude::*;
use diesel::RunQueryDsl;
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
    fn find_by_id(id: PostIdentifier) -> Post;
    fn save(post: Post);
}

impl PostRepository for PostMySqlRepository {
    fn find_by_id(id: PostIdentifier) -> Post {
        let connection = establish_connection();

        let post = posts_schema::dsl::posts
            .find(id.to_string())
            .first::<PostModel>(&connection)
            .expect("Error loading posts");

        return Post::new(
            PostIdentifier { identifier: Ulid::from_string(&post.id).unwrap() },
            PostTitle { title: post.title }
        )
    }
    fn save(post: Post) {
        let connection = establish_connection();
        let new_post = NewPosts {
            id: post.identifier(),
            title: post.title.title,
        };
        diesel::insert_into(posts_schema::table)
            .values(&new_post)
            .execute(&connection)
            .expect("Error saving new posts.");
    }
}
