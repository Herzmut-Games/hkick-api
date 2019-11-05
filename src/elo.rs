const K_FACTOR: f32 = 32_f32;

pub enum GameResult {
    Win,
    Draw,
    Loss,
    Score(f32),
}

impl GameResult {
    fn get_val(&self) -> &f32 {
        match self {
            GameResult::Win => &1_f32,
            GameResult::Draw => &0.5,
            GameResult::Loss => &0_f32,
            GameResult::Score(i) => &i,
        }
    }
}

fn rating_change(score: GameResult, exp_score: f32) -> i32 {
    (K_FACTOR * (score.get_val() - exp_score)) as i32
}

pub fn expected_score(rating_a: i32, rating_b: i32) -> f32 {
    1_f32 / (1_f32 + 10_f32.powf((rating_b - rating_a) as f32 / 400_f32))
}

pub fn calc_elo(
    rating_a: i32,
    rating_b: i32,
    game_score: GameResult,
) -> (i32, i32) {
    let s_b = GameResult::Score(1_f32 - game_score.get_val());

    let expected_a = expected_score(rating_a, rating_b);
    let expected_b = 1_f32 - expected_a;

    let new_a = rating_a + rating_change(game_score, expected_a);
    let new_b = rating_b + rating_change(s_b, expected_b);

    (new_a, new_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_calc_game() {
        let john = 1700;
        let paul = 1800;

        let (john, paul) = calc_elo(paul, john, GameResult::Win);
        assert_eq!(john, 1811);
        assert_eq!(paul, 1689);
    }

    #[test]
    fn should_calc_expected() {
        let john = 1700;
        let paul = 1800;
        let chance = expected_score(john, paul) * 100_f32;
        let expected = 35.9935;

        assert_eq!(chance, expected);
    }
}
