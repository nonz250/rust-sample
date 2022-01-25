use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_repoditory::PostMySqlRepository;
use crate::domain::post::post_repoditory::PostRepository;
use crate::domain::post::post_title::PostTitle;

pub struct UpdatePostInput {
    pub identifier: PostIdentifier,
    pub title: PostTitle
}

pub struct UpdatePost {}

impl UpdatePost {
    pub fn process(input: UpdatePostInput) -> Result<(), String> {
        let mut post = match PostMySqlRepository::find_by_id(input.identifier) {
            Ok(post) => post,
            Err(err) => return Err(err)
        };
        post.change_title(input.title);
        PostMySqlRepository::save(post);
        return Ok(());
    }
}