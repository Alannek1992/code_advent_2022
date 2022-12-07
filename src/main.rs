use std::fs;

use puzzles::{first_puzzle::FirstPuzzle, second_puzzle::SecondPuzzle};

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
    let puzzles:Vec<Box<dyn Solution>> = vec![Box::new(FirstPuzzle::new()), Box::new(SecondPuzzle::new())];
    
    puzzles.iter().for_each(|puzzle| puzzle.solution());
}
