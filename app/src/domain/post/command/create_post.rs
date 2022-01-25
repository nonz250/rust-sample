use crate::domain::post::post_factory::PostFactory;
use crate::domain::post::post_factory::PostMySqlFactory;
use crate::domain::post::post_repoditory::PostMySqlRepository;
use crate::domain::post::post_repoditory::PostRepository;
use crate::domain::post::post_title::PostTitle;

pub struct CreatePostInput {
    pub title: PostTitle,
}

pub struct CreatePost {}

impl CreatePost {
    pub fn process(input: CreatePostInput) {
        let mut post = PostMySqlFactory::new_post();
        post.change_title(input.title);
        PostMySqlRepository::save(post);
    }
}