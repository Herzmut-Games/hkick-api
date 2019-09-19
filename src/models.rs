#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
    pub rating: i32,
}

#[derive(Queryable)]
pub struct Team {
    pub player1: i32,
    pub player2: i32,
    pub rating: i32,
}
