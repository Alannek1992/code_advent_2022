use crate::{util::print_solution, PuzzleInfo, Solution};

// for more details check the https://adventofcode.com/2022/day/3
pub struct ThirdPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for ThirdPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            &self.total_priority(),
            &self.priority_of_group_badges(),
        );
    }
}

struct Rucksack<'a> {
    tools: &'a str,
}

impl<'a> Rucksack<'a> {
    fn duplicates_within_compartments(&self) -> Vec<char> {
        let left = &self.tools[..self.tools.len() / 2];
        let right = &self.tools[self.tools.len() / 2..];

        let mut duplicates: Vec<char> = left.chars().filter(|c| right.contains(*c)).collect();
        duplicates.dedup();
        duplicates
    }

    fn common_tool_within_rucksacks(rucksacks: Vec<Self>) -> Option<char> {
        let mut rucksacks_iter = rucksacks.iter();
        let first_rucksack = rucksacks_iter.next().expect("there must be rucksack");

        for tool in first_rucksack.tools.chars() {
            let mut is_common = true;
            let mut other_rucksacks_iter = rucksacks_iter.clone();

            loop {
                match other_rucksacks_iter.next() {
                    Some(r) => {
                        is_common = r.tools.contains([tool]);
                        if !is_common {
                            break;
                        }
                    }
                    None => break,
                };
            }
            if is_common {
                return Some(tool);
            }
        }

        None
    }
}

impl ThirdPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Third Puzzle - Rucksack Reorganization", "./inputs/3.txt"),
        }
    }

    fn total_priority(&self) -> i32 {
        self.get_rucksacks()
            .iter()
            .map(|r| {
                r.duplicates_within_compartments()
                    .iter()
                    .map(|t| self.priority_per_tool(*t))
                    .sum::<i32>()
            })
            .sum()
    }

    fn priority_of_group_badges(&self) -> i32 {
        let mut rucksacks = self.get_rucksacks().into_iter().peekable();
        let mut total_priority = 0;

        while rucksacks.peek().is_some() {
            let chunk: Vec<Rucksack> = rucksacks.by_ref().take(3).collect();
            let common_tool =
                Rucksack::common_tool_within_rucksacks(chunk).expect("There must be common item");
            total_priority += self.priority_per_tool(common_tool);
        }

        total_priority
    }

    fn priority_per_tool(&self, tool_as_char: char) -> i32 {
        if tool_as_char.is_lowercase() {
            tool_as_char as i32 - 96
        } else {
            tool_as_char as i32 - 38
        }
    }

    fn get_rucksacks(&self) -> Vec<Rucksack> {
        self.puzzle
            .input
            .lines()
            .map(|line| Rucksack { tools: line.trim() })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_priority() {
        assert_eq!(
            157,
            ThirdPuzzle {
                puzzle: get_puzzle_info(),
            }
            .total_priority()
        );
    }

    #[test]
    fn priority_of_group_badges() {
        assert_eq!(
            70,
            ThirdPuzzle {
                puzzle: get_puzzle_info(),
            }
            .priority_of_group_badges()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "vJrwpWtwJgWrhcsFMMfFFhFp
                jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                PmmdzqPrVvPwwTWBwg
                wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                ttgJtRGJQctTZtZT
                CrZsJsPPZsGzwwsLwLmpwMDw",
            ),
        }
    }
}
