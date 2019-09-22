use crate::models::Match;
use crate::schema::matches;
use crate::schema::matches::dsl::*;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "matches"]
pub struct NewMatch {
    pub team_1: i32,
    pub team_2: i32,
}

// #[post("/", format = "json", data = "<player_ids>")]
// pub fn create(conn: DbConn, player_ids: Json<[i32; 4]>) -> Result<Status, diesel::result::Error> {
//     match diesel::insert_into(players)
//         .values(&player_data.into_inner())
//         .execute(&*conn)
//     {
//         Ok(_) => Ok(Status::new(200, "User created")),
//         Err(e) => Err(e),
//     }
// }
