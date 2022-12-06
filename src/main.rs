use puzzles::first_puzzle::FirstPuzzle;

mod puzzles;
mod util;

pub trait Puzzle {
    fn display_solution(&self);
}

fn main() {
    let puzzles: Vec<Box<dyn Puzzle>> = vec![Box::new(FirstPuzzle::new())];

    puzzles.iter().for_each(|p| p.display_solution());
}
