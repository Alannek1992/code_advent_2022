use crate::{util::print_solution, PuzzleInfo, Solution};

// for more details check the https://adventofcode.com/2022/day/2
pub struct SecondPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for SecondPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            &self.total_score(&self.first_decode_strategy()),
            &self.total_score(&self.second_decode_strategy()),
        );
    }
}

struct Game {
    opponent_play: GameMoveVariant,
    my_play: GameMoveVariant,
}
#[derive(PartialEq, Clone)]
enum GameMoveVariant {
    ROCK,
    PAPER,
    SCISSORS,
}

enum GameResult {
    WIN,
    DRAW,
    LOSE,
}

impl Game {
    fn new(opponent_play: GameMoveVariant, my_play: GameMoveVariant) -> Self {
        Self {
            opponent_play,
            my_play,
        }
    }

    fn get_desired_move_variant(
        first_player_move: &GameMoveVariant,
        desired_outcome: GameResult,
    ) -> GameMoveVariant {
        match desired_outcome {
            GameResult::WIN => match first_player_move {
                GameMoveVariant::ROCK => GameMoveVariant::PAPER,
                GameMoveVariant::PAPER => GameMoveVariant::SCISSORS,
                GameMoveVariant::SCISSORS => GameMoveVariant::ROCK,
            },
            GameResult::LOSE => match first_player_move {
                GameMoveVariant::ROCK => GameMoveVariant::SCISSORS,
                GameMoveVariant::PAPER => GameMoveVariant::ROCK,
                GameMoveVariant::SCISSORS => GameMoveVariant::PAPER,
            },
            GameResult::DRAW => first_player_move.clone(),
        }
    }

    fn evaluate_result(&self) -> GameResult {
        if self.opponent_play == self.my_play {
            GameResult::DRAW
        } else {
            match self.opponent_play {
                GameMoveVariant::ROCK => {
                    if self.my_play == GameMoveVariant::PAPER {
                        GameResult::WIN
                    } else {
                        GameResult::LOSE
                    }
                }
                GameMoveVariant::PAPER => {
                    if self.my_play == GameMoveVariant::SCISSORS {
                        GameResult::WIN
                    } else {
                        GameResult::LOSE
                    }
                }
                GameMoveVariant::SCISSORS => {
                    if self.my_play == GameMoveVariant::ROCK {
                        GameResult::WIN
                    } else {
                        GameResult::LOSE
                    }
                }
            }
        }
    }
}

impl SecondPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Second Puzzle - Rock Paper Scissors", "./inputs/2.txt"),
        }
    }

    fn total_score<F: Fn(char, char) -> (GameMoveVariant, GameMoveVariant)>(
        &self,
        decode_strategy: F,
    ) -> i32 {
        self.get_games(decode_strategy)
            .iter()
            .map(|game| self.score_per_game(game))
            .sum()
    }

    fn first_decode_strategy(
        &self,
    ) -> Box<dyn Fn(char, char) -> (GameMoveVariant, GameMoveVariant)> {
        Box::new(|opponent_play_enc, my_play_enc| {
            let opponent_play = match opponent_play_enc {
                'A' => GameMoveVariant::ROCK,
                'B' => GameMoveVariant::PAPER,
                'C' => GameMoveVariant::SCISSORS,
                _ => panic!("Cannot decrypt the game variant"),
            };
            let my_play = match my_play_enc {
                'X' => GameMoveVariant::ROCK,
                'Y' => GameMoveVariant::PAPER,
                'Z' => GameMoveVariant::SCISSORS,
                _ => panic!("Cannot decrypt the game variant"),
            };

            (opponent_play, my_play)
        })
    }

    fn second_decode_strategy(
        &self,
    ) -> Box<dyn Fn(char, char) -> (GameMoveVariant, GameMoveVariant)> {
        Box::new(|opponent_play_enc, my_play_enc| {
            let opponent_play = match opponent_play_enc {
                'A' => GameMoveVariant::ROCK,
                'B' => GameMoveVariant::PAPER,
                'C' => GameMoveVariant::SCISSORS,
                _ => panic!("Cannot decrypt the game variant"),
            };
            let my_play = match my_play_enc {
                'X' => Game::get_desired_move_variant(&opponent_play, GameResult::LOSE),
                'Z' => Game::get_desired_move_variant(&opponent_play, GameResult::WIN),
                'Y' => Game::get_desired_move_variant(&opponent_play, GameResult::DRAW),

                _ => panic!("Cannot decrypt the game variant"),
            };

            (opponent_play, my_play)
        })
    }

    fn score_per_game(&self, game: &Game) -> i32 {
        let points_per_shape = match game.my_play {
            GameMoveVariant::ROCK => 1,
            GameMoveVariant::PAPER => 2,
            GameMoveVariant::SCISSORS => 3,
        };
        let points_per_result = match game.evaluate_result() {
            GameResult::LOSE => 0,
            GameResult::DRAW => 3,
            GameResult::WIN => 6,
        };
        points_per_shape + points_per_result
    }

    fn get_games<F: Fn(char, char) -> (GameMoveVariant, GameMoveVariant)>(
        &self,
        decode: F,
    ) -> Vec<Game> {
        self.puzzle
            .input
            .lines()
            .map(|line| {
                let line_without_spaces = line
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>();
                let mut moves = line_without_spaces.chars();
                let opponent_play = moves.next().expect("Opponent play does not exist");
                let my_play = moves.next().expect("My play does not exist");
                let (opponent_play, my_play) = decode(opponent_play, my_play);
                Game::new(opponent_play, my_play)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_score_first_strategy() {
        let puzzle = SecondPuzzle {
            puzzle: get_puzzle_info(),
        };
        assert_eq!(15, puzzle.total_score(puzzle.first_decode_strategy()))
    }

    #[test]
    fn total_score_second_strategy() {
        let puzzle = SecondPuzzle {
            puzzle: get_puzzle_info(),
        };
        assert_eq!(12, puzzle.total_score(puzzle.second_decode_strategy()))
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "A Y
                B X
                C Z",
            ),
        }
    }
}
