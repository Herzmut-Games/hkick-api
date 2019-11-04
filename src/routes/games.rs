use crate::errors::ApiError;
use crate::models::games::*;
use crate::models::matches::*;
use crate::rating::*;
use crate::schema::games::dsl::*;
use crate::schema::matches::dsl::matches;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

#[get("/<id_match>/games")]
pub fn get_all_for_match(
    conn: DbConn,
    id_match: i32,
) -> Result<JsonValue, ApiError> {
    matches
        .find(id_match)
        .first::<Match>(&*conn)
        .map_err(|_| ApiError::new("Could not find match", 404))?
        .get_games(&*conn)
        // Map resulting games to json result
        .map(|g| json!(g))
}

// This is mounted in /matches
#[put("/<id_match>/games", format = "json", data = "<game_result>")]
pub fn place_game(
    conn: DbConn,
    id_match: i32,
    game_result: Json<GameResult>,
) -> Result<Status, ApiError> {
    let new_game = NewGame::new(game_result.into_inner(), id_match);

    diesel::insert_into(games)
        .values(&new_game)
        .execute(&*conn)
        .map_err(|_| ApiError::new("Could not insert game", 500))
        .and_then(|_| {
            games
                .filter(match_id.eq(new_game.match_id))
                .first(&*conn)
                .map_err(|_| {
                    ApiError::new("Could not find newly inserted game", 500)
                })
        })
        .and_then(|inserted_game: Game| {
            let (winner, loser) = inserted_game.get_winner_and_loser(&*conn)?;
            update_team_ratings(winner, loser, &*conn)
                .and_then(|_| update_player_ratings(winner, loser, &*conn))
                .and_then(|_| Ok(Status::raw(200)))
        })
}
