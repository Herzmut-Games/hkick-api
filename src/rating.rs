use crate::elo::game;
use crate::elo::GameResult;
use crate::errors::ApiError;
use crate::models::players::*;
use crate::models::teams::*;
use diesel::SqliteConnection;

pub fn update_team_ratings(
    winner: Team,
    loser: Team,
    conn: &SqliteConnection,
) -> Result<(), ApiError> {
    let (winner, loser) = calc_team_rating(winner, loser);

    winner
        .update_in_db(conn)
        .and_then(|_| loser.update_in_db(conn))?;

    Ok(())
}

fn calc_team_rating(mut winner: Team, mut loser: Team) -> (Team, Team) {
    let (winner_rating, loser_rating) =
        game(winner.rating, loser.rating, GameResult::Win);

    winner.rating = winner_rating;
    loser.rating = loser_rating;

    (winner, loser)
}

pub fn update_player_ratings(
    winner: Team,
    loser: Team,
    conn: &SqliteConnection,
) -> Result<(), ApiError> {
    let winning_players = winner.get_players(conn)?;
    let losing_players = loser.get_players(conn)?;

    for p in winning_players.iter() {
        let mut p = p.clone();
        p.rating = calc_player_rating(&p, GameResult::Win, &losing_players);
        p.update_in_db(conn)?;
    }

    for p in losing_players.iter() {
        let mut p = p.clone();
        p.rating = calc_player_rating(&p, GameResult::Loss, &winning_players);
        p.update_in_db(conn)?;
    }

    Ok(())
}

fn calc_player_rating(
    player: &Player,
    result: GameResult,
    opponents: &[Player; 2],
) -> i32 {
    let average_rating: i32 = (opponents[0].rating + opponents[1].rating) / 2;
    let (player_rating, _) = game(player.rating, average_rating, result);

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
            rating: 1000,
        };

        let opponents = [
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 1200,
            },
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 800,
            },
        ];

        let winner_rating =
            calc_player_rating(&winner, GameResult::Win, &opponents);

        assert_eq!(winner_rating, 1016);
    }

    #[test]
    fn should_lower_elo_on_loss() {
        let winner = Player {
            id: 1,
            first_name: String::new(),
            surname: String::new(),
            nickname: String::new(),
            rating: 1000,
        };

        let opponents = [
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 1200,
            },
            Player {
                id: 1,
                first_name: String::new(),
                surname: String::new(),
                nickname: String::new(),
                rating: 800,
            },
        ];

        let winner_rating =
            calc_player_rating(&winner, GameResult::Loss, &opponents);

        assert_eq!(winner_rating, 984);
    }
}
