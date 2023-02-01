use std::fs;

use puzzles::{
    eighth_puzzle::EighthPuzzle, eleventh_puzzle::EleventhPuzzle, fifth_puzzle::FifthPuzzle,
    first_puzzle::FirstPuzzle, fourth_puzzle::FourthPuzzle, ninth_puzzle::NinthPuzzle,
    second_puzzle::SecondPuzzle, seventh_puzzle::SeventhPuzzle, sixth_puzzle::SixthPuzzle,
    tenth_puzzle::TenthPuzzle, third_puzzle::ThirdPuzzle, thirteenth_puzzle::ThirteenthPuzzle,
    twelfth_puzzle::TwelfthPuzzle, fourteenth_puzzle::FourteenthPuzzle,
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
        Box::new(EighthPuzzle::new()),
        Box::new(NinthPuzzle::new()),
        Box::new(TenthPuzzle::new()),
        Box::new(EleventhPuzzle::new()),
        Box::new(TwelfthPuzzle::new()),
        Box::new(ThirteenthPuzzle::new()),
        Box::new(FourteenthPuzzle::new()),
    ];

    puzzles.iter().for_each(|puzzle| puzzle.solution());
}
