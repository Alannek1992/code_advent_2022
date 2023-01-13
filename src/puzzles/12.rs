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
}

struct Location {
    line: u8,
    col: u8,
    tag: char,
}

impl Location {
    fn new(line: u8, col: u8, tag: char) -> Self {
        Self { line, col, tag }
    }

    fn is_destination(&self) -> bool {
        self.tag == 'E'
    }

    fn is_start(&self) -> bool {
        self.tag == 'S'
    }
}

impl TwelfthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new(
                "Twelfth Puzzle - Hill Climbing Algorithm",
                "./inputs/12.txt",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

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
