use crate::models::*;
use crate::schema::players;
use crate::schema::players::dsl::*;
use crate::DbConn;
use diesel::prelude::*;
use skill_rating::elo;

/**
 * Expects given players to be sorted by rating descending
 */
fn get_teams(to_match: &Vec<Player>) -> [Team; 2] {}

pub fn find_teams(conn: DbConn, player_ids: [i32; 4]) {
    let players_to_match = players
        .filter(id.eq(player_ids[0]))
        .or_filter(id.eq(player_ids[1]))
        .or_filter(id.eq(player_ids[2]))
        .or_filter(id.eq(player_ids[3]))
        .order(rating.desc())
        .load::<Player>(&*conn)
        .expect("Error loading players");
}
