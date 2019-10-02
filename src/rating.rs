use crate::errors::ApiError;
use crate::models::players::*;
use crate::models::teams::*;
use crate::schema::teams::dsl::*;

use diesel::prelude::*;
use diesel::SqliteConnection;
use skill_rating::elo;

const K_FACTOR: u32 = 32;

pub fn update_team_ratings(
    winner: Team,
    loser: Team,
    conn: &SqliteConnection,
) -> Result<usize, ApiError> {
    let (winner, loser) = calc_team_rating(winner, loser);

    update_team_rating(winner, conn)
        .and_then(|_| update_team_rating(loser, conn))
}

fn update_team_rating(
    team: Team,
    conn: &SqliteConnection,
) -> Result<usize, ApiError> {
    diesel::update(teams.filter(id.eq(team.id)))
        .set(rating.eq(team.rating))
        .execute(conn)
        .map_err(|_| ApiError::new("Could not update team rating", 500))
}

fn calc_team_rating(mut winner: Team, mut loser: Team) -> (Team, Team) {
    let (winner_rating, loser_rating) =
        elo::game(winner.rating, loser.rating, elo::WIN, K_FACTOR, K_FACTOR);

    winner.rating = winner_rating;
    loser.rating = loser_rating;

    (winner, loser)
}

fn calc_player_rating(
    mut player: Player,
    result: f32,
    opponents: (Player, Player),
) -> Player {
    let average_rating: i32 = (opponents.0.rating + opponents.1.rating) / 2;
    let (player_rating, _) =
        elo::game(player.rating, average_rating, result, K_FACTOR, K_FACTOR);

    player.rating = player_rating;

    player
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

        let opponents = (
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
        );

        let winner = calc_player_rating(winner, elo::WIN, opponents);

        assert_eq!(winner.rating, 1016);
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

        let opponents = (
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
        );

        let winner = calc_player_rating(winner, elo::LOSS, opponents);

        assert_eq!(winner.rating, 984);
    }
}
