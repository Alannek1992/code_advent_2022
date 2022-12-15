use std::fs;

use puzzles::{
    fifth_puzzle::FifthPuzzle, first_puzzle::FirstPuzzle, fourth_puzzle::FourthPuzzle,
    second_puzzle::SecondPuzzle, sixth_puzzle::SixthPuzzle, third_puzzle::ThirdPuzzle, seventh_puzzle::SeventhPuzzle,
};

mod puzzles;
mod util;

pub struct PuzzleInfo {
    name: String,
    input: String,
}

impl PuzzleInfo {
    pub fn new(name: &str, file_path: &str) -> Self {
        Self {
            name: String::from(name),
            input: fs::read_to_string(file_path).expect("Not able to read the file"),
        }
    }
}
pub trait Solution {
    fn solution(&self);
}

fn main() {
    let puzzles: Vec<Box<dyn Solution>> = vec![
        Box::new(FirstPuzzle::new()),
        Box::new(SecondPuzzle::new()),
        Box::new(ThirdPuzzle::new()),
        Box::new(FourthPuzzle::new()),
        Box::new(FifthPuzzle::new()),
        Box::new(SixthPuzzle::new()),
        Box::new(SeventhPuzzle::new()),
    ];

    puzzles.iter().for_each(|puzzle| puzzle.solution());
}
