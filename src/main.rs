use puzzles::first_puzzle::FirstPuzzle;
use util::read_input_file;

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
            input: read_input_file(file_path),
        }
    }
}
pub trait Solution {
    fn solution(&self);
}

fn main() {
    let puzzles: Vec<Box<dyn Solution>> = vec![Box::new(FirstPuzzle::new())];

    puzzles.iter().for_each(|p| p.solution());
}
