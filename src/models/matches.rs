use crate::models::games::*;
use crate::schema::matches;

#[derive(serde_derive::Deserialize, Clone, Queryable, serde_derive::Serialize)]
pub struct Match {
    pub id: i32,
    pub team_1: i32,
    pub team_2: i32,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(serde_derive::Deserialize)]
pub struct MatchDetails {
    pub match_data: Match,
    pub game_data: Vec<Game>,
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_2: i32,
}
