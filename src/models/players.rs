use crate::schema::players;

#[derive(Clone, Queryable, serde_derive::Serialize)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
    pub rating: i32,
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub first_name: String,
    pub surname: String,
    pub nickname: String,
}
