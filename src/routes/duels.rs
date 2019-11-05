use crate::errors::ApiError;
use crate::models::duels::*;
use crate::schema::duels::dsl::{
    duels as table_duels, id as col_id, player_1 as col_player_1,
    player_2 as col_player_2,
};
use crate::DbConn;

use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

fn get_duel_id(
    new_duel: &NewDuel,
    conn: &SqliteConnection,
) -> Result<i32, ApiError> {
    match table_duels
        .filter(col_player_1.eq(new_duel.player_1))
        .filter(col_player_2.eq(new_duel.player_2))
        .select(col_id)
        .order(col_id.desc())
        .limit(1)
        .load(conn)
    {
        Ok(duel_id) => Ok(*duel_id.first().unwrap()),
        Err(_) => Err(ApiError::new("Could not find duel", 404)),
    }
}

#[post("/", format = "json", data = "<player_ids>")]
pub fn create(
    conn: DbConn,
    player_ids: Json<[i32; 2]>,
) -> Result<JsonValue, ApiError> {
    let [player_1, player_2] = player_ids.into_inner();

    let new_duel = NewDuel { player_1, player_2 };

    diesel::insert_into(table_duels)
        .values(&new_duel)
        .execute(&*conn)
        .map_err(|_| ApiError::new("Could not create duel", 500))
        .and_then(|_| get_duel_id(&new_duel, &*conn))
        .map(|duel_id| json!({ "id": duel_id }))
}

#[get("/<id_duel>")]
pub fn get_by_id(conn: DbConn, id_duel: i32) -> Result<JsonValue, ApiError> {
    table_duels
        .find(id_duel)
        .first::<Duel>(&*conn)
        .map(|g| json!(g))
        .map_err(|_| ApiError::new("Could not find duel", 404))
}

#[get("/")]
pub fn get_all(conn: DbConn) -> Result<JsonValue, ApiError> {
    table_duels
        .load::<Duel>(&*conn)
        .map(|result| json!(result))
        .map_err(|_| ApiError::new("Error fetching duels", 500))
}
