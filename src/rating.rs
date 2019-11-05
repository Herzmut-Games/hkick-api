use crate::elo::calc_elo;
use crate::elo::GameResult;
use crate::errors::ApiError;
use crate::models::{
    duels::Duel, games::Game, matches::Match, players::Player, teams::Team,
};
use crate::schema::duels::dsl::duels as table_duels;
use crate::schema::matches::dsl::matches as table_matches;

use diesel::{prelude::*, SqliteConnection};

fn match_get_winner_loser(
    game: &Game,
    conn: &SqliteConnection,
) -> Result<(Team, Team), ApiError> {
    let match_to_check = game.match_id.ok_or_else(|| {
        ApiError::new("Game does not have a match assigned", 500)
    })?;

    let match_to_check = table_matches
        .find(match_to_check)
        .first::<Match>(&*conn)
        .map_err(|_| ApiError::new("Could not find match", 404))?;

    if game.score_1 > game.score_2 {
        Ok((
            match_to_check.get_team_1(conn)?,
            match_to_check.get_team_2(conn)?,
        ))
    } else {
        Ok((
            match_to_check.get_team_2(conn)?,
            match_to_check.get_team_1(conn)?,
        ))
    }
}

pub fn update_team_ratings(
    game: &Game,
    conn: &SqliteConnection,
) -> Result<(Team, Team), ApiError> {
    let (winner, loser) = match_get_winner_loser(game, conn)?;
    let (winner, loser) = calc_team_rating(winner, loser);

    winner
        .update_in_db(conn)
        .and_then(|_| loser.update_in_db(conn))?;

    Ok((winner, loser))
}

fn calc_team_rating(mut winner: Team, mut loser: Team) -> (Team, Team) {
    let (winner_rating, loser_rating) =
        calc_elo(winner.rating, loser.rating, GameResult::Win);

    winner.rating = winner_rating;
    loser.rating = loser_rating;

    (winner, loser)
}

fn duel_get_winner_loser(
    game: &Game,
    conn: &SqliteConnection,
) -> Result<(Player, Player), ApiError> {
    let duel_to_check = game.duel_id.ok_or_else(|| {
        ApiError::new("Game does not have a duel assigned", 500)
    })?;

    let duel_to_check =
        table_duels
            .find(duel_to_check)
            .first::<Duel>(&*conn)
            .map_err(|_| ApiError::new("Could not find duel", 404))?;

    if game.score_1 > game.score_2 {
        Ok((
            duel_to_check.get_player_1(conn)?,
            duel_to_check.get_player_2(conn)?,
        ))
    } else {
        Ok((
            duel_to_check.get_player_2(conn)?,
            duel_to_check.get_player_1(conn)?,
        ))
    }
}

pub fn duel_update_player_ratings(
    game: &Game,
    conn: &SqliteConnection,
) -> Result<(), ApiError> {
    let (winner, loser) = {
        let (mut winner, mut loser) = duel_get_winner_loser(game, conn)?;
        let (winner_rating, loser_rating) =
            calc_elo(winner.solo_rating, loser.solo_rating, GameResult::Win);

        winner.solo_rating = winner_rating;
        loser.solo_rating = loser_rating;

        (winner, loser)
    };

    winner.update_solo_rating(conn)?;
    loser.update_solo_rating(conn)?;

    Ok(())
}

pub fn match_update_player_ratings(
    winner: Team,
    loser: Team,
    conn: &SqliteConnection,
) -> Result<(), ApiError> {
    let winning_players = winner.get_players(conn)?;
    let losing_players = loser.get_players(conn)?;

    for player in winning_players.iter() {
        let mut player = player.clone();
        player.team_rating =
            calc_player_rating(&player, GameResult::Win, &losing_players);
        player.update_team_rating(conn)?;
    }

    for player in losing_players.iter() {
        let mut player = player.clone();
        player.team_rating =
            calc_player_rating(&player, GameResult::Loss, &winning_players);
        player.update_team_rating(conn)?;
    }

    Ok(())
}

fn calc_player_rating(
    player: &Player,
    result: GameResult,
    opponents: &[Player; 2],
) -> i32 {
    let highest_opponent_rating: i32 =
        if opponents[0].team_rating > opponents[1].team_rating {
            opponents[0].team_rating
        } else {
            opponents[1].team_rating
        };

    let (player_rating, _) =
        calc_elo(player.team_rating, highest_opponent_rating, result);

    player_rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_update_team_ratings() {
        let winner = Team {
            id: 1,
            player_1: 1,
            player_2: 2,
            rating: 1000,
        };

        let loser = Team {
            id: 2,
            player_1: 3,
            player_2: 4,
            rating: 1000,
        };

        let (winner, loser) = calc_team_rating(winner, loser);

        assert_eq!(winner.rating, 1016);
        assert_eq!(loser.rating, 984);
    }

    #[test]
    fn should_rise_elo_on_win() {
        let winner = Player {
            id: 1,
            first_name: String::new(),
            surname: String::new(),
            nickname: String::new(),
            team_rating: 1000,
            solo_rating: 1000,
        };

        let opponents = [
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 1200,
                solo_rating: 1200,
            },
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 800,
                solo_rating: 800,
            },
        ];

        let winner_rating =
            calc_player_rating(&winner, GameResult::Win, &opponents);

        assert_eq!(winner_rating, 1024);
    }

    #[test]
    fn should_lower_elo_on_loss() {
        let winner = Player {
            id: 1,
            first_name: String::new(),
            surname: String::new(),
            nickname: String::new(),
            team_rating: 1000,
            solo_rating: 1000,
        };

        let opponents = [
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 1200,
                solo_rating: 1200,
            },
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                team_rating: 800,
                solo_rating: 800,
            },
        ];

        let winner_rating =
            calc_player_rating(&winner, GameResult::Loss, &opponents);

        assert_eq!(winner_rating, 993);
    }
}
