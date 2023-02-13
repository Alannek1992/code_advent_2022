use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct FifteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FifteenthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.positions_not_containing_beacon(2000000),
            0,
        );
    }
}

type Coordinate = (i32, i32);

#[derive(Debug)]
struct Line {
    starting_coordinate: Coordinate,
    ending_coordinate: Coordinate,
    coordinates_coverage: i32,
}

impl Line {
    fn new(starting_coordinate: Coordinate, ending_coordinate: Coordinate) -> Self {
        Self {
            starting_coordinate,
            ending_coordinate,
            coordinates_coverage: Self::calc_coordinate_coverage(
                starting_coordinate,
                ending_coordinate,
            ),
        }
    }

    fn calc_coordinate_coverage(
        starting_coordinate: Coordinate,
        ending_coordinate: Coordinate,
    ) -> i32 {
        (starting_coordinate.0 - ending_coordinate.0).abs()
            + (starting_coordinate.1 - ending_coordinate.1).abs()
    }

    fn is_in_coordinates_coverage(&self, coordinate: Coordinate) -> bool {
        Self::calc_coordinate_coverage(self.starting_coordinate, coordinate)
            <= self.coordinates_coverage
    }
}

impl FifteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new(
                "Fifteenth Puzzle - Beacon Exclusion Zone",
                "./inputs/15.txt",
            ),
        }
    }

    fn positions_not_containing_beacon(&self, y: i32) -> u32 {
        let sensors_and_beacons = self.read_sensors();
        let x_coordinates_of_beacons: Vec<i32> = sensors_and_beacons
            .iter()
            .map(|line| line.ending_coordinate.0)
            .sorted()
            .collect();
        let mut positions_not_containing_beacon = 0;

        for x in *x_coordinates_of_beacons.first().unwrap() + 1
            ..*x_coordinates_of_beacons.last().unwrap()
        {
            for line in sensors_and_beacons.iter() {
                if line.is_in_coordinates_coverage((x, y)) {
                    positions_not_containing_beacon += 1;
                    break;
                }
            }
        }

        positions_not_containing_beacon
    }

    fn read_sensors(&self) -> Vec<Line> {
        let re_coordinates = Regex::new(r".*x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();
        self.puzzle
            .input
            .lines()
            .map(|line| {
                let captures = re_coordinates.captures(line.trim()).unwrap();
                Line::new(
                    (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                    (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn positions_not_containing_beacon() {
        assert_eq!(
            26,
            FifteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .positions_not_containing_beacon(10)
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
                Sensor at x=9, y=16: closest beacon is at x=10, y=16
                Sensor at x=13, y=2: closest beacon is at x=15, y=3
                Sensor at x=12, y=14: closest beacon is at x=10, y=16
                Sensor at x=10, y=20: closest beacon is at x=10, y=16
                Sensor at x=14, y=17: closest beacon is at x=10, y=16
                Sensor at x=8, y=7: closest beacon is at x=2, y=10
                Sensor at x=2, y=0: closest beacon is at x=2, y=10
                Sensor at x=0, y=11: closest beacon is at x=2, y=10
                Sensor at x=20, y=14: closest beacon is at x=25, y=17
                Sensor at x=17, y=20: closest beacon is at x=21, y=22
                Sensor at x=16, y=7: closest beacon is at x=15, y=3
                Sensor at x=14, y=3: closest beacon is at x=15, y=3
                Sensor at x=20, y=1: closest beacon is at x=15, y=3",
            ),
        }
    }
}
