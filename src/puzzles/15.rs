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

type Coordinate = (X, Y);

trait CoordinateMerging {
    fn merge(&mut self, next_coordinate: Coordinate) -> Result<(), ()>;
}

impl CoordinateMerging for Coordinate {
    fn merge(&mut self, next_coordinate: Self) -> Result<(), ()> {
        if next_coordinate.0 - 1 <= self.1 {
            if self.0 > next_coordinate.0 {
                self.0 = next_coordinate.0;
            }
            if self.1 < next_coordinate.1 {
                self.1 = next_coordinate.1;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}

type X = i32;
type Y = i32;

struct Rhombus {
    center: Coordinate,
    coverage: i32,
}

impl Rhombus {
    fn new(sensor: Coordinate, beacon: Coordinate) -> Self {
        Self {
            center: sensor,
            coverage: Self::calculate_coverage(sensor, beacon),
        }
    }

    fn calculate_coverage(first_coordinate: Coordinate, second_coordinate: Coordinate) -> i32 {
        (first_coordinate.0 - second_coordinate.0).abs()
            + (first_coordinate.1 - second_coordinate.1).abs()
    }

    fn calculate_boundary_with_restriction(
        &self,
        value: i32,
        restriction_value: i32,
        restriction_kind: RestrictionKind,
    ) -> i32 {
        match restriction_kind {
            RestrictionKind::GreaterEq => {
                if value < restriction_value {
                    restriction_value
                } else {
                    value
                }
            }
            RestrictionKind::LessEq => {
                if value > restriction_value {
                    restriction_value
                } else {
                    value
                }
            }
        }
    }

    fn get_x_coordinates_for_y(
        &self,
        y: Y,
        x_restriction: Option<Coordinate>,
    ) -> Option<Coordinate> {
        let diff = (self.center.1 - y).abs();
        let coverage = self.coverage - diff;

        if coverage < 0 {
            return None;
        };

        match x_restriction {
            Some(restriction) => Some((
                self.calculate_boundary_with_restriction(
                    self.center.0 - coverage,
                    restriction.0,
                    RestrictionKind::GreaterEq,
                ),
                self.calculate_boundary_with_restriction(
                    self.center.0 + coverage,
                    restriction.1,
                    RestrictionKind::LessEq,
                ),
            )),
            None => Some((self.center.0 - coverage, self.center.0 + coverage)),
        }
    }
}

enum RestrictionKind {
    GreaterEq,
    LessEq,
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

    fn tuning_frequency(&self) -> i64 {
        let (x_restriction, y_restriction) = self.get_restricted_area();

        for line_no in y_restriction.0..=y_restriction.1 {
            let mut lines: Vec<Coordinate> = vec![];
            for rhombus in self.rhombuses.iter() {
                match rhombus.get_x_coordinates_for_y(line_no, Some(x_restriction)) {
                    Some(line) => lines.push(line),
                    None => continue,
                }
            }
            lines.sort_by(|a, b| a.0.cmp(&b.0));
            let mut coord = *lines.get(0).unwrap();
            for l in lines {
                match coord.merge(l) {
                    Ok(()) => {}
                    Err(()) => return (l.0 - 1) as i64 * 4000000 + line_no as i64,
                }
            }
        }
        unreachable!()
    }

    fn positions_not_containing_beacon(&self, y: Y) -> i64 {
        let x_coordinates: Vec<i32> = self
            .rhombuses
            .iter()
            .flat_map(|r| match r.get_x_coordinates_for_y(y, None) {
                Some(line) => vec![line.0, line.1],
                None => vec![],
            })
            .sorted()
            .collect();

        (*x_coordinates.get(0).unwrap()..*x_coordinates.get(x_coordinates.len() - 1).unwrap()).len()
            as i64
    }

    fn get_restricted_area(&self) -> (Coordinate, Coordinate) {
        let (mut starting_x, mut ending_x) = (i32::MAX, i32::MIN);
        let (mut starting_y, mut ending_y) = (i32::MAX, i32::MIN);

        for sensor in self.sensors.iter() {
            if sensor.0 >= 0 && sensor.0 < starting_x {
                starting_x = sensor.0
            }
            if sensor.0 <= 4000000 && sensor.0 > ending_x {
                ending_x = sensor.0
            }
            if sensor.1 >= 0 && sensor.1 < starting_y {
                starting_y = sensor.1
            }
            if sensor.1 <= 4000000 && sensor.1 > ending_y {
                ending_y = sensor.1
            }
        }

        ((starting_x, ending_x), (starting_y, ending_y))
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
