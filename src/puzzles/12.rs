use std::collections::{HashMap, HashSet};

use crate::PuzzleInfo;

pub struct TwelfthPuzzle {
    puzzle: PuzzleInfo,
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
    distance: u8,
}

impl QItem {
    fn new(location: Location, movement: Movement, distance: u8) -> Self {
        Self {
            location,
            movement,
            distance,
        }
    }

    fn is_destination(&self) -> bool {
        self.movement == 'E'
    }

    fn is_start(&self) -> bool {
        self.movement == 'S'
    }

    fn get_adjacent_locations(&self) -> Vec<Location> {
        let mut adjacents = Vec::new();
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

    fn bfs_steps_needed(&self) {
        let heightmap = self.read_heightmap();
        let mut visited_locations = HashSet::new();
        let mut queue = Queue::new();

        let starting_location = QItem::new((0, 0), 'S', 0);
        visited_locations.insert(starting_location.location);
        queue.enqueue(starting_location);

        while !queue.is_empty() {
            let item = queue.dequeue().unwrap();

        }
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
        TwelfthPuzzle {
            puzzle: get_puzzle_info(),
        }
        .bfs_steps_needed();
        assert!(false);
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
