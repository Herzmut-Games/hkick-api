use crate::errors::ApiError;
use crate::models::duels::Duel;
use crate::models::games::{Game, GameResult, NewGame};
use crate::models::matches::Match;
use crate::rating::*;
use crate::schema::duels::dsl::duels as table_duels;
use crate::schema::games::dsl::{
    duel_id as col_duel_id, games as table_games, match_id as col_match_id,
};
use crate::schema::matches::dsl::matches as table_matches;
use crate::DbConn;

use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

/**
 ** Mounted in /matches
 **/
#[get("/<id_match>/games")]
pub fn match_get_all(
    conn: DbConn,
    id_match: i32,
) -> Result<JsonValue, ApiError> {
    table_matches
        .find(id_match)
        .first::<Match>(&*conn)
        .map_err(|_| ApiError::new("Could not find match", 404))?
        .get_games(&*conn)
        // Map resulting games to json result
        .map(|g| json!(g))
}

#[put("/<id_match>/games", format = "json", data = "<game_result>")]
pub fn match_place_game(
    conn: DbConn,
    id_match: i32,
    game_result: Json<GameResult>,
) -> Result<Status, ApiError> {
    let new_game = NewGame::new(game_result.into_inner(), Some(id_match), None);

    diesel::insert_into(table_games)
        .values(&new_game)
        .execute(&*conn)
        .map_err(|_| ApiError::new("Could not insert game", 500))
        .and_then(|_| {
            table_games
                .filter(col_match_id.eq(new_game.match_id))
                .first(&*conn)
                .map_err(|_| {
                    ApiError::new("Could not find newly inserted game", 500)
                })
        })
        .and_then(|inserted_game: Game| {
            update_team_ratings(&inserted_game, &*conn)
                .and_then(|(winner, loser)| {
                    match_update_player_ratings(winner, loser, &*conn)
                })
                .and_then(|_| Ok(Status::raw(200)))
        })
}

/**
 ** Mounted in /duels
 **/
#[get("/<id_duel>/games")]
pub fn duel_get_all(conn: DbConn, id_duel: i32) -> Result<JsonValue, ApiError> {
    table_duels
        .find(id_duel)
        .first::<Duel>(&*conn)
        .map_err(|_| ApiError::new("Could not find match", 404))?
        .get_games(&*conn)
        // Map resulting games to json result
        .map(|g| json!(g))
}

#[put("/<id_duel>/games", format = "json", data = "<game_result>")]
pub fn duel_place_game(
    conn: DbConn,
    id_duel: i32,
    game_result: Json<GameResult>,
) -> Result<Status, ApiError> {
    let new_game = NewGame::new(game_result.into_inner(), None, Some(id_duel));

    diesel::insert_into(table_games)
        .values(&new_game)
        .execute(&*conn)
        .map_err(|_| ApiError::new("Could not insert game", 500))
        .and_then(|_| {
            table_games
                .filter(col_duel_id.eq(new_game.duel_id))
                .first(&*conn)
                .map_err(|_| {
                    ApiError::new("Could not find newly inserted game", 500)
                })
        })
        .and_then(|inserted_game: Game| {
            duel_update_player_ratings(&inserted_game, &*conn)
                .and_then(|_| Ok(Status::raw(200)))
        })
}
