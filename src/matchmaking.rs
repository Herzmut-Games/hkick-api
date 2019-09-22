use crate::models::*;
use crate::schema::players;
use crate::schema::players::dsl::*;
use crate::DbConn;
use diesel::prelude::*;
use skill_rating::elo;

/**
 * Expects to_match to be sorted descending by rating
 */
// fn get_teams(to_match: Vec<Player>) -> [Team; 2] {}

pub fn find_teams(conn: DbConn, player_ids: &[i32; 4]) {
    let players_to_match = players
        .filter(id.eq(player_ids[0]))
        .or_filter(id.eq(player_ids[1]))
        .or_filter(id.eq(player_ids[2]))
        .or_filter(id.eq(player_ids[3]))
        .order(rating.desc())
        .load::<Player>(&*conn)
        .expect("Error loading players from database");

    let players_to_match: [Player; 4] = match players_to_match.len() {
        4 => [
            players_to_match[0].clone(),
            players_to_match[1].clone(),
            players_to_match[2].clone(),
            players_to_match[3].clone(),
        ],
        _ => panic!("Error getting players for teams"),
    };
}
