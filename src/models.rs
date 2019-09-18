#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
}
