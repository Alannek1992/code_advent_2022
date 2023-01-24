use core::panic;
use std::collections::{HashMap, HashSet};

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct TwelfthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for TwelfthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.bfs_steps_needed(), 0);
    }
}

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.items.len() > 0 {
            return Some(self.items.remove(0));
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

struct QItem {
    location: Location,
    movement: Movement,
    distance: u32,
}

impl QItem {
    fn new(location: Location, movement: Movement, distance: u32) -> Self {
        Self {
            location,
            movement,
            distance,
        }
    }

    fn is_destination(&self) -> bool {
        self.movement == 'E'
    }

    fn get_adjacent_locations(&self) -> Vec<Location> {
        let mut adj_locations = vec![
            (self.location.0, self.location.1 + 1),
            (self.location.0 + 1, self.location.1),
        ];
        if self.location.0 > 0 {
            adj_locations.push((self.location.0 - 1, self.location.1));
        }
        if self.location.1 > 0 {
            adj_locations.push((self.location.0, self.location.1 - 1));
        }

        adj_locations
    }

    fn interpret_elevation(elevation: char) -> u8 {
        match elevation {
            'E' => 'z' as u8,
            'S' => 'a' as u8,
            _ => elevation as u8,
        }
    }
}

type Location = (u8, u8);
type Movement = char;

impl TwelfthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new(
                "Twelfth Puzzle - Hill Climbing Algorithm",
                "./inputs/12.txt",
            ),
        }
    }

    fn bfs_steps_needed(&self) -> u32 {
        let heightmap = self.read_heightmap();
        let mut visited_locations = HashSet::new();
        let mut queue = Queue::new();

        let starting_location = heightmap.iter().find(|(_, value)| **value == 'S').unwrap();

        let starting_location = QItem::new(*starting_location.0, *starting_location.1, 0);
        visited_locations.insert(starting_location.location);
        queue.enqueue(starting_location);

        while !queue.is_empty() {
            let item = queue.dequeue().unwrap();

            if item.is_destination() {
                return item.distance;
            }

            for adjacent_location in item.get_adjacent_locations() {
                match heightmap.get(&adjacent_location) {
                    Some(movement) => {
                        if QItem::interpret_elevation(item.movement) + 1
                            >= QItem::interpret_elevation(*movement)
                            && !visited_locations.contains(&adjacent_location)
                        {
                            queue.enqueue(QItem::new(
                                adjacent_location,
                                *movement,
                                item.distance + 1,
                            ));
                            visited_locations.insert(adjacent_location);
                        }
                    }
                    None => continue,
                }
            }
        }
        panic!("Solution not found!");
    }

    fn read_heightmap(&self) -> HashMap<Location, Movement> {
        let mut heightmap = HashMap::new();

        for (line_no, line) in self.puzzle.input.lines().enumerate() {
            for (col_no, col) in line.trim().chars().enumerate() {
                heightmap.insert((line_no as u8, col_no as u8), col);
            }
        }

        heightmap
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn bfs_steps_needed() {
        assert_eq!(
            31,
            TwelfthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .bfs_steps_needed()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
            ),
        }
    }
}
