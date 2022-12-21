use std::collections::HashSet;

use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct NinthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for NinthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.get_grid(1).positions_visited_by_tail(),
            0,
        );
    }
}

type Position = (i32, i32);
enum Movement {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl Movement {
    fn get_value(&self) -> i32 {
        match self {
            Movement::Up(n) => *n,
            Movement::Right(n) => *n,
            Movement::Down(n) => *n,
            Movement::Left(n) => *n,
        }
    }
}

struct GridOfPositions {
    head: Position,
    tail: Tail,
    movements: Vec<Movement>,
}

struct Tail {
    last_knot_pos: Position,
    knots: Vec<Position>,
}

impl Tail {
    fn new(length: usize) -> Self {
        Self {
            last_knot_pos: (0, 0),
            knots: vec![(0, 0); length],
        }
    }
}

impl GridOfPositions {
    fn build_grid(movements: Vec<Movement>, tail_length: usize) -> Self {
        Self {
            head: (0, 0),
            tail: Tail::new(tail_length),
            movements,
        }
    }

    fn positions_visited_by_tail(&mut self) -> u32 {
        let mut visited_positions = HashSet::new();
        // starting position
        visited_positions.insert((0, 0));
        for movement in self.movements.iter() {
            for _ in 0..movement.get_value() {
                match movement {
                    Movement::Up(_) => self.head.0 += 1,
                    Movement::Right(_) => self.head.1 += 1,
                    Movement::Down(_) => self.head.0 -= 1,
                    Movement::Left(_) => self.head.1 -= 1,
                }

                let mut previous_knot = self.head;
                let knots = &mut self.tail.knots;
                let knots_len = knots.len();
                for (idx, knot) in knots.iter_mut().enumerate() {
                    let is_line_gap = (previous_knot.0 - knot.0).abs() > 1;
                    let is_col_gap = (previous_knot.1 - knot.1).abs() > 1;

                    if !(is_line_gap || is_col_gap) {
                        break;
                    } else if is_line_gap {
                        knot.1 = previous_knot.1;

                        // problem is related to direction recognition
                        if let Movement::Up(_) = movement {
                            knot.0 += 1;
                        } else {
                            knot.0 -= 1;
                        }
                    } else if is_col_gap {
                        knot.0 = previous_knot.0;
                        if let Movement::Right(_) = movement {
                            knot.1 += 1;
                        } else {
                            knot.1 -= 1;
                        }
                    }

                    if idx == knots_len - 1 {
                        self.tail.last_knot_pos = (knot.0, knot.1);
                        visited_positions.insert(self.tail.last_knot_pos);
                    }

                    previous_knot = *knot;
                }
            }
        }
        visited_positions.len() as u32
    }
}

impl NinthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Ninth Puzzle - Rope Bridge", "./inputs/9.txt"),
        }
    }

    fn get_grid(&self, tail_length: usize) -> GridOfPositions {
        let mut movements = Vec::new();
        let re_movement = Regex::new(r"([A-Z]) (\d+)").unwrap();

        self.puzzle.input.lines().for_each(|line| {
            let line = line.trim();
            let movement_captures = re_movement.captures(line).unwrap();
            let distance: i32 = *&movement_captures[2].parse().unwrap();
            let movement = match &movement_captures[1].chars().next().unwrap() {
                'U' => Movement::Up(distance),
                'R' => Movement::Right(distance),
                'D' => Movement::Down(distance),
                'L' => Movement::Left(distance),
                _ => unreachable!(),
            };
            movements.push(movement);
        });

        GridOfPositions::build_grid(movements, tail_length)
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn positions_visited_by_tail() {
        assert_eq!(
            13,
            NinthPuzzle {
                puzzle: get_puzzle_info(
                    "R 4
                U 4
                L 3
                D 1
                R 4
                D 1
                L 5
                R 2"
                ),
            }
            .get_grid(1)
            .positions_visited_by_tail()
        );
    }

    #[test]
    fn positions_visited_by_last_knot_of_extended_tail() {
        assert_eq!(
            36,
            NinthPuzzle {
                puzzle: get_puzzle_info(
                    "R 5
                    U 8
                    L 8
                    D 3
                    R 17
                    D 10
                    L 25
                    U 20"
                ),
            }
            .get_grid(9)
            .positions_visited_by_tail()
        );
    }

    fn get_puzzle_info(input: &str) -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(input),
        }
    }
}
