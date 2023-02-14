use std::collections::HashSet;

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
            0,
        );
    }
}

type Coordinate = (i32, i32);
type Line = (X, X);
type X = i32;
type Y = i32;

struct Area {
    sensors: Vec<Coordinate>,
    beacons: Vec<Coordinate>,
}

impl Area {
    fn new() -> Self {
        Self {
            sensors: Vec::new(),
            beacons: Vec::new(),
        }
    }

    fn calc_coverage(starting_coordinate: Coordinate, ending_coordinate: Coordinate) -> i32 {
        (starting_coordinate.0 - ending_coordinate.0).abs()
            + (starting_coordinate.1 - ending_coordinate.1).abs()
    }

    fn add_sensor_and_beacon(&mut self, sensor: Coordinate, beacon: Coordinate) {
        self.sensors.push(sensor);
        self.beacons.push(beacon);
    }

    fn extend_to_y(&self, y: Y) -> HashSet<Line> {
        let mut coordinates = HashSet::new();
        self.sensors
            .iter()
            .zip(self.beacons.iter())
            .for_each(|(sensor, beacon)| {
                let coverage = Self::calc_coverage(*sensor, *beacon);
                let coverage = coverage - (sensor.1 - y).abs();

                if coverage > 0 {
                    coordinates.insert((sensor.0 - coverage, sensor.0 + coverage));
                }
            });
        coordinates
    }

    fn tuning_frequency(&self) -> i32 {
        let mut starting_y = 0;
        let mut starting_x = 0;
        let mut ending_x = 0;
        self.sensors.iter().for_each(|s| {
            if s.1 >= 0 && s.1 < starting_y {
                starting_y = s.1
            }

            if s.0 >= 0 && s.0 <= 4000000 {
                if s.0 < starting_x {
                    starting_x = s.0;
                }

                if s.0 > ending_x {
                    ending_x = s.0
                }
            }
        });

        loop {
            let test: Vec<(i32, i32)> = self
                .extend_to_y(starting_y)
                .into_iter()
                .filter(|l| l.0 >= starting_x && l.0 <= ending_x)
                .collect();
            println!("{:?}", test);

            break;
        }

        10
    }

    fn positions_not_containing_beacon(&self, y: Y) -> i32 {
        let mut min = 0;
        let mut max = 0;
        self.extend_to_y(y).iter().for_each(|(start, end)| {
            if *start < min {
                min = *start;
            }

            if *end > max {
                max = *end;
            }
        });

        (min..max).len() as i32
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
