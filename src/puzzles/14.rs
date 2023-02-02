use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct FourteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FourteenthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.sand_coming_to_the_rest(), 0);
    }
}

type Coordinate = (i16, i16);

trait FillLine {
    fn bridge_the_gap(&self, coordinate: &Coordinate) -> Vec<Coordinate>;
}

impl FillLine for Coordinate {
    fn bridge_the_gap(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let mut missing_coordinates = vec![];
        let increment_or_decrement = |first: i16, second: i16| {
            if first > second {
                first - 1
            } else {
                first + 1
            }
        };
        let get_missing_coordinate = |first: &Coordinate, second: &Coordinate| {
            let is_line_gap = (first.0 - second.0).abs() > 1;
            let is_col_gap = (first.1 - second.1).abs() > 1;

            if is_line_gap {
                Some((increment_or_decrement(first.0, second.0), first.1))
            } else if is_col_gap {
                Some((first.0, increment_or_decrement(first.1, second.1)))
            } else {
                None
            }
        };

        match get_missing_coordinate(self, coordinate) {
            Some(c) => {
                missing_coordinates.push(c);
                loop {
                    match get_missing_coordinate(missing_coordinates.last().unwrap(), coordinate) {
                        Some(mc) => missing_coordinates.push(mc),
                        None => break,
                    };
                }

                missing_coordinates
            }
            None => missing_coordinates,
        }
    }
}

struct Tringle {
    coordinates: HashSet<Coordinate>,
    height: i16,
}

impl Tringle {
    fn new(coordinates: HashSet<Coordinate>) -> Self {
        let height = *coordinates.iter().map(|(_, y)| y).max().unwrap();

        Self {
            coordinates,
            height,
        }
    }

    fn spread_the_coordinates(&mut self) {
        let starting_height = *self.coordinates.iter().map(|(_, y)| y).min().unwrap();
        let mut starting_coordinate: Coordinate = (500, starting_height - 1);

        loop {
            match starting_coordinate.spread(&self.coordinates, self.height) {
                Ok(c) => {
                    self.coordinates.insert(c);
                }
                Err(kind) => match kind {
                    ErrorKind::NotSpace => starting_coordinate.1 -= 1,
                    ErrorKind::FallingForever => break,
                },
            }
        }
    }
}

trait Sand {
    fn spread(
        &self,
        existing_tiles: &HashSet<Coordinate>,
        max_height: i16,
    ) -> Result<Coordinate, ErrorKind>;

    fn movement(&self, kind: MoveKind) -> Coordinate;
}

impl Sand for Coordinate {
    fn spread(
        &self,
        existing_tiles: &HashSet<Coordinate>,
        max_height: i16,
    ) -> Result<Coordinate, ErrorKind> {
        let left_occupied = existing_tiles.contains(&(self.0 - 1, self.1 + 1));
        let right_occupied = existing_tiles.contains(&(self.0 + 1, self.1 + 1));
        let down_occupied = existing_tiles.contains(&(self.0, self.1 + 1));

        if self.1 == max_height {
            return Err(ErrorKind::FallingForever);
        }

        if left_occupied && right_occupied && down_occupied {
            if existing_tiles.contains(self) {
                return Err(ErrorKind::NotSpace);
            } else {
                return Ok(*self);
            }
        }

        let next_coordinate = if !down_occupied {
            self.movement(MoveKind::Down)
        } else if !left_occupied {
            self.movement(MoveKind::LeftDown)
        } else {
            self.movement(MoveKind::RightDown)
        };

        next_coordinate.spread(existing_tiles, max_height)
    }

    fn movement(&self, kind: MoveKind) -> Coordinate {
        match kind {
            MoveKind::LeftDown => (self.0 - 1, self.1 + 1),
            MoveKind::RightDown => (self.0 + 1, self.1 + 1),
            MoveKind::Down => (self.0, self.1 + 1),
        }
    }
}

enum MoveKind {
    LeftDown,
    RightDown,
    Down,
}

enum ErrorKind {
    FallingForever,
    NotSpace,
}

impl FourteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Fourteenth Puzzle - Regolith Reservoir", "./inputs/14.txt"),
        }
    }

    fn sand_coming_to_the_rest(&self) -> usize {
        let coordinates = self.scan_path();
        let origin_len = coordinates.len();
        let mut triangle = Tringle::new(coordinates);
        triangle.spread_the_coordinates();
        triangle.coordinates.len() - origin_len
    }

    fn scan_path(&self) -> HashSet<Coordinate> {
        let re_number = Regex::new(r"(\d+,\d+)").unwrap();
        let mut coordinates = HashSet::new();
        self.puzzle.input.lines().for_each(|line| {
            let mut path: Vec<Coordinate> = vec![];
            let line = line.trim();
            re_number.captures_iter(line).for_each(|c| {
                let coordinate = c[0]
                    .split_terminator(",")
                    .map(|n| n.parse().unwrap())
                    .next_tuple()
                    .unwrap();
                if path.is_empty() {
                    path.push(coordinate);
                } else {
                    path.extend(coordinate.bridge_the_gap(path.last().unwrap()));
                    path.push(coordinate);
                }
            });
            coordinates.extend(path);
        });

        coordinates
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn sand_to_rest_till_source_is_blocked() {
        assert_eq!(
            93,
            /*FourteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sand_coming_to_the_rest(FloorKind::Solid)*/
            3
        );
    }

    #[test]
    fn sand_coming_to_the_rest() {
        assert_eq!(
            24,
            FourteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sand_coming_to_the_rest()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9
                ",
            ),
        }
    }
}
