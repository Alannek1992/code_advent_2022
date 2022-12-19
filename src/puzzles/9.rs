use std::collections::HashSet;

use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct NinthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for NinthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, 0, 0);
    }
}

type VisitedPosition = (u32, u32);
enum Movement {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}

struct GridOfVisitedPositions {
    visited_positions: HashSet<VisitedPosition>,
}

impl GridOfVisitedPositions {
    fn build_grid(movements: Vec<Movement>) -> Self {
        let visited_positions = HashSet::new();

        Self { visited_positions }
    }
}

impl NinthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Ninth Puzzle - Rope Bridge", "./inputs/9.txt"),
        }
    }

    fn get_grid(&self) -> GridOfVisitedPositions {
        let mut movements = Vec::new();
        let re_movement = Regex::new(r"([A-Z]) (\d+)").unwrap();

        self.puzzle.input.lines().for_each(|line| {
            let line = line.trim();
            let movement_captures = re_movement.captures(line).unwrap();
            let distance: u32 = *&movement_captures[2].parse().unwrap();
            let test = &movement_captures[1];
            let movement = match &movement_captures[1].chars().next().unwrap() {
                'U' => Movement::Up(distance),
                'R' => Movement::Right(distance),
                'D' => Movement::Down(distance),
                'L' => Movement::Left(distance),
                _ => unreachable!(),
            };
            movements.push(movement);
        });

        GridOfVisitedPositions::build_grid(movements)
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn test_part_one() {
        NinthPuzzle {
            puzzle: get_puzzle_info(),
        }
        .get_grid();
        assert!(false);
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "R 4
                U 4
                L 3
                D 1
                R 4
                D 1
                L 5
                R 2",
            ),
        }
    }
}
