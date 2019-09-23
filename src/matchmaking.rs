use crate::models::*;
use crate::schema::players::dsl::id as players_id;
use crate::schema::players::dsl::rating as players_rating;
use crate::schema::players::dsl::*;
use crate::schema::teams;
use crate::schema::teams::dsl::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

pub enum MatchmakingError {
    Create,
    GetPlayers,
    WrongPlayerAmount,
}

#[derive(serde_derive::Deserialize, Insertable)]
#[table_name = "teams"]
pub struct NewTeam {
    pub player_1: i32,
    pub player_2: i32,
}

fn create_team(
    conn: &SqliteConnection,
    p_1: &Player,
    p_2: &Player,
) -> Result<Team, MatchmakingError> {
    let t = NewTeam {
        player_1: p_1.id,
        player_2: p_2.id,
    };

    match diesel::insert_into(teams).values(t).execute(&*conn) {
        Ok(_) => Ok(get_or_create_team(conn, p_1, p_2)?.to_owned()),
        Err(_) => Err(MatchmakingError::Create),
    }
}

fn get_or_create_team(
    conn: &SqliteConnection,
    p_1: &Player,
    p_2: &Player,
) -> Result<Team, MatchmakingError> {
    let t = teams
        .filter(player_1.eq(p_1.id))
        .filter(player_2.eq(p_2.id))
        .load::<Team>(conn)
        .unwrap();

    match t.len() {
        1 => Ok(*t.first().unwrap()),
        _ => create_team(conn, p_1, p_2),
    }
}

pub fn find_teams(
    conn: &SqliteConnection,
    player_ids: &[i32; 4],
) -> Result<(Team, Team), MatchmakingError> {
    let players_to_match = match players
        .filter(players_id.eq(player_ids[0]))
        .or_filter(players_id.eq(player_ids[1]))
        .or_filter(players_id.eq(player_ids[2]))
        .or_filter(players_id.eq(player_ids[3]))
        .order(players_rating.desc())
        .load::<Player>(conn)
    {
        Err(_) => return Err(MatchmakingError::GetPlayers),
        Ok(ps) => ps,
    };

    let players_to_match: [Player; 4] = match players_to_match.len() {
        4 => [
            players_to_match[0].clone(),
            players_to_match[1].clone(),
            players_to_match[2].clone(),
            players_to_match[3].clone(),
        ],
        _ => return Err(MatchmakingError::WrongPlayerAmount),
    };

    Ok((
        get_or_create_team(conn, &players_to_match[0], &players_to_match[3])?,
        get_or_create_team(conn, &players_to_match[1], &players_to_match[2])?,
    ))
}
