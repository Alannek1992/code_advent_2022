use first_puzzle::FirstPuzzle;

mod first_puzzle;
mod second_puzzle;
mod util;

pub trait Puzzle {
    fn display_solution(&self);
}

fn main() {
    let puzzles: Vec<Box<dyn Puzzle>> = vec![Box::new(FirstPuzzle::new())];

    puzzles.iter().for_each(|p| p.display_solution());
}
