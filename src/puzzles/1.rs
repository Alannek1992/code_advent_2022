use crate::{util::print_solution, PuzzleInfo, Solution};

// for more details check the https://adventofcode.com/2022/day/1
pub struct FirstPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FirstPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.most_calories(),
            self.sum_top_three_calories(),
        );
    }
}

impl FirstPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("First Puzzle - Calories", "./inputs/1.txt"),
        }
    }
    fn most_calories(&self) -> i32 {
        *self
            .get_calories_per_elf()
            .iter()
            .max()
            .expect("the vector with calories is empty")
    }

    fn sum_top_three_calories(&self) -> i32 {
        let mut calories_per_elf = self.get_calories_per_elf();
        calories_per_elf.sort();
        *&calories_per_elf[calories_per_elf.len() - 3..].iter().sum()
    }

    fn get_calories_per_elf(&self) -> Vec<i32> {
        let mut calories_per_elv = Vec::new();
        let mut acc_calories_per_elf = 0;
        let mut lines = self.puzzle.input.lines();

        loop {
            match lines.next() {
                Some(line) => {
                    if line.trim().is_empty() {
                        calories_per_elv.push(acc_calories_per_elf);
                        acc_calories_per_elf = 0;
                        continue;
                    }
                    acc_calories_per_elf += line
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse the line from input into integer");
                }
                None => {
                    calories_per_elv.push(acc_calories_per_elf);
                    break;
                }
            }
        }
        calories_per_elv
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn most_calories() {
        // the most calories carried by Elf should be 24000
        assert_eq!(
            24000,
            FirstPuzzle {
                puzzle: get_puzzle_info()
            }
            .most_calories()
        );
    }

    #[test]
    fn sum_top_three_calories() {
        // The sum of the Calories carried by these three elves is 45000
        assert_eq!(
            45000,
            FirstPuzzle {
                puzzle: get_puzzle_info()
            }
            .sum_top_three_calories()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "1000
                2000
                3000
                
                4000
                
                5000
                6000
                
                7000
                8000
                9000
                
                10000",
            ),
        }
    }
}
