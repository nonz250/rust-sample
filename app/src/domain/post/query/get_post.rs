use diesel::prelude::*;
use crate::domain::post::post::Post;
use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;
use crate::models::Post as PostModel;
use crate::schema::posts as posts_schema;
use crate::utils::establish_connection;

pub struct GetPostMySqlQuery {}

pub struct GetPostMockQuery {}

pub trait GetPostQuery {
    fn get_post() -> Post;
}

impl GetPostQuery for GetPostMySqlQuery {
    fn get_post() -> Post {
        let connection = establish_connection();

        let result = posts_schema::dsl::posts
            .first::<PostModel>(&connection)
            .expect("Error loading posts");

        let post_id = PostIdentifier {
            identifier: result.id
        };

        let post_title = PostTitle {
            title: result.title.to_string()
        };

        let post = Post {
            identifier: post_id,
            title: post_title
        };

        return post;
    }
}

impl GetPostQuery for GetPostMockQuery {
    fn get_post() -> Post {
        let post_id = PostIdentifier {
            identifier: 1
        };
        let post_title = PostTitle {
            title: "mock".to_string()
        };
        let post = Post {
            identifier: post_id,
            title: post_title
        };
        return post;
    }
}
