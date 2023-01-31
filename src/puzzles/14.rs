use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::PuzzleInfo;

pub struct FourteenthPuzzle {
    puzzle: PuzzleInfo,
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

type Sand = Coordinate;

trait Spread {
    fn spread_to_coordinate(
        &self,
        starting_coordinate: Coordinate,
        available_coordinates: &HashSet<Coordinate>,
        existing_sand: &mut HashSet<Sand>,
    ) -> Result<(), ()>;
}

// confused lines and cols - TODO fix

impl Spread for Sand {
    fn spread_to_coordinate(
        &self,
        starting_coordinate: Coordinate,
        available_coordinates: &HashSet<Coordinate>,
        existing_sand: &mut HashSet<Sand>,
    ) -> Result<(), ()> {
        let max_line_allowed = available_coordinates.iter().max().unwrap().0;

        if starting_coordinate.0 <= max_line_allowed {
            return Err(());
        }

        match available_coordinates.get(&(starting_coordinate.0 - 1, starting_coordinate.1)) {
            Some(c) => {
                if !existing_sand.contains(&starting_coordinate) {
                    existing_sand.insert(starting_coordinate);
                    return Ok(());
                }

                // left diagonal
                match self.spread_to_coordinate(
                    (c.0, c.1 - 1),
                    available_coordinates,
                    existing_sand,
                ) {
                    Ok(()) => return Ok(()),
                    Err(()) => match self.spread_to_coordinate(
                        (c.0, c.1 + 1),
                        available_coordinates,
                        existing_sand,
                    ) {
                        Ok(()) => return Ok(()),
                        Err(()) => return Err(()),
                    },
                }
            }
            None => self.spread_to_coordinate(
                (starting_coordinate.0 - 1, starting_coordinate.1),
                available_coordinates,
                existing_sand,
            ),
        }
    }
}

impl FourteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Fourteenth Puzzle - Regolith Reservoir", "./inputs/14.txt"),
        }
    }

    fn sand_coming_to_the_rest(&self) -> usize {
        let mut sand_to_rest = HashSet::new();
        let available_coordinates = self.scan_path();
        loop {
            match ((500 as i16, 500 as i16) as Sand).spread_to_coordinate(
                (500, 500),
                &available_coordinates,
                &mut sand_to_rest,
            ) {
                Err(()) => break,
                _ => {}
            }
        }

        println!("{:?}", sand_to_rest);

        10
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
    fn part_one() {
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
503,4 -> 502,4 -> 502,9 -> 494,9",
            ),
        }
    }
}
