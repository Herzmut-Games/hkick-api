use crate::errors::ApiError;
use crate::models::{players::*, teams::*};
use crate::schema::players::dsl::{id as players_id, rating as players_rating, *};
use crate::schema::teams::dsl::*;

use diesel::{prelude::*, SqliteConnection};

fn create_team(conn: &SqliteConnection, new_team: NewTeam) -> Result<Team, ApiError> {
    match diesel::insert_into(teams).values(&new_team).execute(&*conn) {
        Ok(_) => Ok(get_or_create_team(conn, new_team)?.to_owned()),
        Err(e) => {
            println!("SQL Error: {}", e);
            Err(ApiError::new("Could not create team", 500))
        }
    }
}

fn get_or_create_team(conn: &SqliteConnection, new_team: NewTeam) -> Result<Team, ApiError> {
    let t = teams
        .filter(player_1.eq(new_team.player_1))
        .filter(player_2.eq(new_team.player_2))
        .load::<Team>(conn)
        .unwrap();

    match t.len() {
        1 => Ok(*t.first().unwrap()),
        _ => create_team(conn, new_team),
    }
}

fn balance_teams(unordered_players: [Player; 4]) -> (NewTeam, NewTeam) {
    let ordered_players = {
        let mut l = unordered_players;
        l.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());
        l
    };

    (
        NewTeam::new(ordered_players[0].id, ordered_players[3].id),
        NewTeam::new(ordered_players[1].id, ordered_players[2].id),
    )
}

pub fn find_teams(
    conn: &SqliteConnection,
    player_ids: &[i32; 4],
) -> Result<(Team, Team), ApiError> {
    let players_to_match = match players
        .filter(players_id.eq(player_ids[0]))
        .or_filter(players_id.eq(player_ids[1]))
        .or_filter(players_id.eq(player_ids[2]))
        .or_filter(players_id.eq(player_ids[3]))
        .order(players_rating.desc())
        .load::<Player>(conn)
    {
        Err(e) => {
            println!("SQL Error: {}", e);
            return Err(ApiError::new("Could not get all players", 500));
        }
        Ok(ps) => ps,
    };

    let new_teams = match players_to_match.len() {
        4 => balance_teams([
            players_to_match[0].clone(),
            players_to_match[1].clone(),
            players_to_match[2].clone(),
            players_to_match[3].clone(),
        ]),
        _ => return Err(ApiError::new("Did not get four players", 500)),
    };

    Ok((
        get_or_create_team(conn, new_teams.0)?,
        get_or_create_team(conn, new_teams.1)?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_balanced_teams() {
        let unorderd_players = [
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 800,
            },
            Player {
                id: 2,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 1200,
            },
            Player {
                id: 3,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 1100,
            },
            Player {
                id: 4,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 700,
            },
        ];

        let expected = (NewTeam::new(4, 2), NewTeam::new(1, 3));

        assert_eq!(balance_teams(unorderd_players), expected);
    }
}
