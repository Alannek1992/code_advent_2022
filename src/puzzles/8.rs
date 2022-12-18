use crate::PuzzleInfo;

pub struct EighthPuzzle {
    puzzle: PuzzleInfo,
}

type TreeCoordinate = (i32, i32);

struct Tree {
    height: i32,
    coordinate: TreeCoordinate,
}

impl EighthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Eighth Puzzle - Treetop Tree House", "./inputs/8.txt"),
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
                "30373
                25512
                65332
                33549
                35390",
            ),
        }
    }
}
