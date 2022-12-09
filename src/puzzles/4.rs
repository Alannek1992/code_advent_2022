use crate::{util::print_solution, PuzzleInfo, Solution};

// for more details check the https://adventofcode.com/2022/day/4
pub struct FourthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FourthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            &self.fully_contained_pairs(),
            &self.overlapping_pairs(),
        );
    }
}

struct CleanupAssignment {
    first_unit: (i32, i32),
    second_unit: (i32, i32),
}

impl CleanupAssignment {
    fn is_fully_contained(&self) -> bool {
        self.first_unit.0 <= self.second_unit.0 && self.first_unit.1 >= self.second_unit.1
            || self.second_unit.0 <= self.first_unit.0 && self.second_unit.1 >= self.first_unit.1
    }

    fn is_overlapping(&self) -> bool {
        self.first_unit.0 <= self.second_unit.1 && self.second_unit.0 <= self.first_unit.1
    }
}

impl FourthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Fourth Puzzle - Camp Cleanup", "./inputs/4.txt"),
        }
    }

    fn fully_contained_pairs(&self) -> i32 {
        self.get_assignments()
            .iter()
            .filter(|assignment| assignment.is_fully_contained())
            .collect::<Vec<&CleanupAssignment>>()
            .len() as i32
    }

    fn overlapping_pairs(&self) -> i32 {
        self.get_assignments()
            .iter()
            .filter(|assignment| assignment.is_overlapping())
            .collect::<Vec<&CleanupAssignment>>()
            .len() as i32
    }

    fn get_assignments(&self) -> Vec<CleanupAssignment> {
        let mut assignments = Vec::new();
        self.puzzle.input.lines().for_each(|line| {
            let mut numbers = Vec::new();
            let mut number_builder = String::new();
            let chars: Vec<char> = line.chars().collect();
            for (idx, c) in chars.iter().enumerate() {
                if c.is_digit(10) {
                    number_builder.push(*c);
                    if idx == chars.len() - 1 {
                        numbers.push(number_builder.parse::<i32>().unwrap());
                    }
                } else {
                    numbers.push(number_builder.parse::<i32>().unwrap());
                    number_builder.clear();
                }
            }

            let [first_unit_start, first_unit_end, second_unit_start, second_unit_end] =
                <[i32; 4]>::try_from(&numbers[..]).unwrap();

            assignments.push(CleanupAssignment {
                first_unit: (first_unit_start, first_unit_end),
                second_unit: (second_unit_start, second_unit_end),
            })
        });

        assignments
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn fully_contained_pairs() {
        assert_eq!(
            2,
            FourthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .fully_contained_pairs()
        );
    }

    #[test]
    fn overlapping_pairs() {
        assert_eq!(
            4,
            FourthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .overlapping_pairs()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            ),
        }
    }
}
