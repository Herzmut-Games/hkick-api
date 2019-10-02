use crate::{models::games::*, schema::games::dsl::*, DbConn};

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

// This is mounted in /matches
#[put("/<id_match>/games", format = "json", data = "<game_result>")]
pub fn place_game(conn: DbConn, id_match: i32, game_result: Json<GameResult>) -> Status {
    let new_game = NewGame::new(game_result.into_inner(), id_match);

    match diesel::insert_into(games).values(&new_game).execute(&*conn) {
        Ok(_) => Status::new(200, "Game inserted"),
        Err(_) => Status::new(500, "Could not insert Game"),
    }
}
