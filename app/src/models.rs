use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPosts {
    pub title: String,
}
