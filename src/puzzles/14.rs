use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct FourteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FourteenthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.sand_coming_to_the_rest(FloorKind::Infinite),
            0,
        );
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

struct Sand {
    available_coordinates: HashSet<Coordinate>,
    max_allowed_y: i16,
    spreaded_sand: HashSet<Coordinate>,
    floor_kind: FloorKind,
}

impl Sand {
    fn new(available_coordinates: HashSet<Coordinate>, floor_kind: FloorKind) -> Self {
        let mut max_allowed_y = *available_coordinates.iter().map(|(_, y)| y).max().unwrap();
        match floor_kind {
            FloorKind::Solid => max_allowed_y += 2,
            _ => {}
        }

        Self {
            available_coordinates,
            max_allowed_y,
            spreaded_sand: HashSet::new(),
            floor_kind,
        }
    }

    fn populate_spreaded_sand(&mut self) {
        let starting_coordinate = (500, 0);
        loop {
            match self.spread(starting_coordinate) {
                Ok(()) => continue,
                Err(_) => break,
            }
        }
    }

    fn spread(&mut self, coordinate: Coordinate) -> Result<(), ErrorKind> {
        let mut all_coordinates = HashSet::from(self.available_coordinates.clone());
        all_coordinates.extend(self.spreaded_sand.clone());

        if all_coordinates.contains(&coordinate) {
            return Err(ErrorKind::NotSpace);
        }

        if coordinate.1 >= self.max_allowed_y {
            match self.floor_kind {
                FloorKind::Infinite => return Err(ErrorKind::FallingForever),
                FloorKind::Solid => return Err(ErrorKind::NotSpace),
            }
        }

        if coordinate == (500, 0) && all_coordinates.contains(&coordinate) {
            return Err(ErrorKind::BlockedSource);
        }

        match self.floor_kind {
            FloorKind::Solid => {
                if coordinate.1 + 1 == self.max_allowed_y {
                    self.spreaded_sand.insert(coordinate);
                    return Ok(());
                }
            }
            _ => {}
        }

        match all_coordinates.get(&(coordinate.0, coordinate.1 + 1)) {
            Some(c) => {
                if all_coordinates.contains(&(coordinate.0, coordinate.1 + 1))
                    && all_coordinates.contains(&(coordinate.0 - 1, coordinate.1 + 1))
                    && all_coordinates.contains(&(coordinate.0 + 1, coordinate.1 + 1))
                {
                    self.spreaded_sand.insert(coordinate);
                    return Ok(());
                }

                match self.spread((c.0 - 1, c.1)) {
                    Ok(()) => Ok(()),
                    Err(kind) => match kind {
                        ErrorKind::NotSpace => match self.spread((c.0 + 1, c.1)) {
                            Ok(()) => Ok(()),
                            Err(kind) => Err(kind),
                        },
                        ErrorKind::FallingForever => Err(kind),
                        ErrorKind::BlockedSource => Err(kind),
                    },
                }
            }
            None => self.spread((coordinate.0, coordinate.1 + 1)),
        }
    }
}

enum ErrorKind {
    FallingForever,
    NotSpace,
    BlockedSource,
}

enum FloorKind {
    Infinite,
    Solid,
}

impl FourteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Fourteenth Puzzle - Regolith Reservoir", "./inputs/14.txt"),
        }
    }

    fn sand_coming_to_the_rest(&self, kind: FloorKind) -> usize {
        let coordinates = self.scan_path();
        let mut sand = Sand::new(coordinates, kind);
        sand.populate_spreaded_sand();
        sand.spreaded_sand.len()
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
            FourteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sand_coming_to_the_rest(FloorKind::Solid)
        );
    }

    #[test]
    fn sand_coming_to_the_rest() {
        assert_eq!(
            24,
            FourteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sand_coming_to_the_rest(FloorKind::Infinite)
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
            ),
        }
    }
}
