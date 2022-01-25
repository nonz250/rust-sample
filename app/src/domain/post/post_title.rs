pub struct PostTitle {
    title: String
}

impl PostTitle {
    const MAX_LENGTH: usize = 50;

    pub fn new(title: String) -> Result<PostTitle, String> {
        if title.len() > Self::MAX_LENGTH {
            return Err(format!("Title must be less than {} characters.", Self::MAX_LENGTH).to_string());
        }
        return Ok(PostTitle {
            title: title.to_string()
        })
    }

    pub fn title(self) -> String {
        self.title
    }
}
