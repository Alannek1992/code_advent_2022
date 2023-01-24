use crate::PuzzleInfo;

pub struct ThirteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl ThirteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Thirteenth Puzzle - Distress Signal", "./inputs/13.txt"),
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
                "[1,1,3,1,1]
                [1,1,5,1,1]
                
                [[1],[2,3,4]]
                [[1],4]
                
                [9]
                [[8,7,6]]
                
                [[4,4],4,4]
                [[4,4],4,4,4]
                
                [7,7,7,7]
                [7,7,7]
                
                []
                [3]
                
                [[[]]]
                [[]]
                
                [1,[2,[3,[4,[5,6,7]]]],8,9]
                [1,[2,[3,[4,[5,6,0]]]],8,9]",
            ),
        }
    }
}
