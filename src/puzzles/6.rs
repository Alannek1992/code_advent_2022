use std::collections::HashMap;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct SixthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for SixthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.chars_before_first_marker(4),
            self.chars_before_first_marker(14),
        );
    }
}

impl SixthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Sixth Puzzle - Tuning Trouble", "./inputs/6.txt"),
        }
    }

    fn chars_before_first_marker(&self, message_breakpoint: usize) -> usize {
        let mut chars_as_map: HashMap<char, usize> = HashMap::new();

        for (i, c) in self.puzzle.input.chars().enumerate() {
            if chars_as_map.len() == message_breakpoint {
                return i;
            }

            if let Some(existing_value) = chars_as_map.get(&c).cloned() {
                chars_as_map.retain(|_key, value| *value > existing_value);
            }

            chars_as_map.insert(c, i);
        }
        panic!("Invalid input");
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn chars_before_first_marker() {
        assert_eq!(
            11,
            SixthPuzzle {
                puzzle: get_puzzle_info("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
            }
            .chars_before_first_marker(4)
        );

        assert_eq!(
            19,
            SixthPuzzle {
                puzzle: get_puzzle_info("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
            }
            .chars_before_first_marker(14)
        )
    }

    fn get_puzzle_info(input: &str) -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(input),
        }
    }
}
