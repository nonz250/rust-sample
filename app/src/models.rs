use crate::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPosts {
    pub title: String,
}
