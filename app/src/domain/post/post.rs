use crate::domain::post::post_identifier::PostIdentifier;
use crate::domain::post::post_title::PostTitle;

pub struct Post {
    identifier: PostIdentifier,
    pub title: PostTitle
}

impl Post {
    pub fn new(identifier: PostIdentifier, title: PostTitle) -> Post {
        Post {
            identifier: identifier,
            title: title
        }
    }
    pub fn identifier(&self) -> String {
        self.identifier.to_string()
    }
    pub fn change_title(&mut self, title: PostTitle) {
        self.title = title
    }
}
