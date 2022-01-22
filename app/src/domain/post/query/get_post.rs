use diesel::prelude::*;
use serde::Serialize;
use ulid::Ulid;
use crate::models::Post;
use crate::schema::posts as posts_schema;
use crate::utils::establish_connection;

#[derive(Serialize)]
pub struct PostReadModel {
    identifier: String,
    title: String
}

pub struct GetPostMySqlQuery {}

pub struct GetPostMockQuery {}

pub trait GetPostQuery {
    fn get_post() -> Vec<PostReadModel>;
}

impl GetPostQuery for GetPostMySqlQuery {
    fn get_post() -> Vec<PostReadModel> {
        let connection = establish_connection();

        let results = posts_schema::dsl::posts
            .load::<Post>(&connection)
            .expect("Error loading posts");

        let mut posts = Vec::new();

        for result in results {
            posts.push(PostReadModel {
                identifier: result.id,
                title: result.title
            })
        }

        return posts;
    }
}

impl GetPostQuery for GetPostMockQuery {
    fn get_post() -> Vec<PostReadModel> {
        return Vec::from([PostReadModel {
            identifier: Ulid::new().to_string(),
            title: "mock".to_string()
        }]);
    }
}
