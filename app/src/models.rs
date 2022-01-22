use crate::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPosts {
    pub id: String,
    pub title: String,
}
