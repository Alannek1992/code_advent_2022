use crate::PuzzleInfo;

pub struct TwelfthPuzzle {
    puzzle: PuzzleInfo,
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
