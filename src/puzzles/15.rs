use std::collections::HashSet;

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
            self.read_sensors().positions_not_containing_beacon(2000000),
            self.read_sensors().tuning_frequency(),
        );
    }
}

type Line = (Coordinate, Coordinate);
type Coordinate = (X, Y);
type X = i32;
type Y = i32;

struct Rhombus {
    center: Coordinate,
    coverage: i32,
}

impl Rhombus {
    fn new(sensor: Coordinate, beacon: Coordinate) -> Self {
        let coverage = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        Self {
            center: sensor,
            coverage,
        }
    }

    fn get_lines(&self) -> Vec<Line> {
        let left = (self.center.0 - self.coverage, self.center.1);
        let down = (self.center.0, self.center.1 + self.coverage);
        let right = (self.center.0 + self.coverage, self.center.1);
        let up = (self.center.0, self.center.1 - self.coverage);
        vec![(left, down), (down, right), (right, up), (up, left)]
    }

    fn get_x_coordinates_for_line(&self, y: Y) -> Option<Line> {
        let diff = (self.center.1 - y).abs();
        let coverage = self.coverage - diff;

        if coverage < 0 {
            return None;
        };

        Some(((self.center.0 - coverage, y), (self.center.0 + coverage, y)))
    }
}

struct Area {
    sensors: Vec<Coordinate>,
    beacons: Vec<Coordinate>,
    rhombuses: Vec<Rhombus>,
}

impl Area {
    fn new() -> Self {
        Self {
            sensors: vec![],
            beacons: vec![],
            rhombuses: vec![],
        }
    }

    fn add_sensor_and_beacon(&mut self, sensor: Coordinate, beacon: Coordinate) {
        self.sensors.push(sensor);
        self.beacons.push(beacon);
        self.rhombuses.push(Rhombus::new(sensor, beacon))
    }

    fn tuning_frequency(&self) -> i32 {
        let test: Vec<Line> = self.rhombuses.iter().flat_map(|r| r.get_lines()).collect();
        println!("hohohou");
        10
    }

    fn positions_not_containing_beacon(&self, y: Y) -> i32 {
        let x_coordinates: Vec<i32> = self
            .rhombuses
            .iter()
            .flat_map(|r| match r.get_x_coordinates_for_line(y) {
                Some(line) => vec![line.0 .0, line.1 .0],
                None => vec![],
            })
            .sorted()
            .collect();

        (*x_coordinates.get(0).unwrap()..*x_coordinates.get(x_coordinates.len() - 1).unwrap()).len()
            as i32
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

    fn read_sensors(&self) -> Area {
        let re_coordinates = Regex::new(r".*x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();
        let mut area = Area::new();
        self.puzzle.input.lines().for_each(|line| {
            let captures = re_coordinates.captures(line.trim()).unwrap();
            area.add_sensor_and_beacon(
                (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            );
        });

        area
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn tuning_frequency() {
        assert_eq!(
            56000011,
            FifteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .read_sensors()
            .tuning_frequency()
        );
    }

    #[test]
    fn positions_not_containing_beacon() {
        assert_eq!(
            26,
            FifteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .read_sensors()
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
