use crate::errors::ApiError;
use crate::models::{players::*, teams::*};
use crate::schema::players::dsl::{
    id as players_id, players as table_players, team_rating as col_team_rating,
};
use crate::schema::teams::dsl::{
    player_1 as col_player_1, player_2 as col_player_2, teams as table_teams,
};

use diesel::{prelude::*, SqliteConnection};

fn create_team(
    conn: &SqliteConnection,
    new_team: NewTeam,
) -> Result<Team, ApiError> {
    match diesel::insert_into(table_teams)
        .values(&new_team)
        .execute(&*conn)
    {
        Ok(_) => Ok(get_or_create_team(conn, new_team)?.to_owned()),
        Err(e) => {
            println!("SQL Error: {}", e);
            Err(ApiError::new("Could not create team", 500))
        }
    }
}

fn get_or_create_team(
    conn: &SqliteConnection,
    new_team: NewTeam,
) -> Result<Team, ApiError> {
    let teams = table_teams
        .filter(col_player_1.eq(new_team.player_1))
        .filter(col_player_2.eq(new_team.player_2))
        .load::<Team>(conn)
        .unwrap();

    match teams.len() {
        1 => Ok(*teams.first().unwrap()),
        _ => create_team(conn, new_team),
    }
}

fn balance_teams(unordered_players: [Player; 4]) -> (NewTeam, NewTeam) {
    let ordered_players = {
        let mut players = unordered_players;
        players
            .sort_by(|a, b| a.team_rating.partial_cmp(&b.team_rating).unwrap());
        players
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
    let players_to_match = table_players
        .filter(players_id.eq(player_ids[0]))
        .or_filter(players_id.eq(player_ids[1]))
        .or_filter(players_id.eq(player_ids[2]))
        .or_filter(players_id.eq(player_ids[3]))
        .order(col_team_rating.desc())
        .load::<Player>(conn)
        .map_err(|_| ApiError::new("Could not get all players", 500))?;

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
                team_rating: 800,
                solo_rating: 800,
            },
            Player {
                id: 2,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 1200,
                solo_rating: 1200,
            },
            Player {
                id: 3,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 1100,
                solo_rating: 1100,
            },
            Player {
                id: 4,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 700,
                solo_rating: 700,
            },
        ];

        let expected = (NewTeam::new(4, 2), NewTeam::new(1, 3));

        assert_eq!(balance_teams(unorderd_players), expected);
    }
}
