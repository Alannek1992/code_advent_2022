use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct FifthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for FifthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            &self
                .get_storage(Box::new(CraneOldModel {}))
                .crates_on_top_of_stacks(),
            &self
                .get_storage(Box::new(CraneNewModel {}))
                .crates_on_top_of_stacks(),
        );
    }
}

struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn new(items: Vec<char>) -> Self {
        Self { items }
    }

    fn peek(&self) -> Option<&char> {
        self.items.last()
    }
}

#[derive(Debug)]
struct Instruction {
    amount: i32,
    from_stack: usize,
    to_stack: usize,
}

impl Instruction {
    fn new(amount: i32, from_stack: usize, to_stack: usize) -> Self {
        Self {
            amount,
            from_stack,
            to_stack,
        }
    }
}

struct Storage {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
    crane_strategy: Box<dyn CraneStrategy>,
}

impl Storage {
    fn new(
        stacks: Vec<Stack>,
        instructions: Vec<Instruction>,
        crane_strategy: Box<dyn CraneStrategy>,
    ) -> Self {
        Self {
            stacks,
            instructions,
            crane_strategy,
        }
    }

    fn crates_on_top_of_stacks(&mut self) -> String {
        self.crane_strategy
            .execute_instructions(&self.instructions, &mut self.stacks);
        let mut crates_on_top = String::new();
        for stack in self.stacks.iter() {
            match stack.peek() {
                Some(c) => crates_on_top.push(*c),
                None => continue,
            };
        }
        crates_on_top
    }
}

trait CraneStrategy {
    fn execute_instructions(&self, instructions: &Vec<Instruction>, stacks: &mut Vec<Stack>);
}

struct CraneOldModel {}

impl CraneStrategy for CraneOldModel {
    fn execute_instructions(&self, instructions: &Vec<Instruction>, stacks: &mut Vec<Stack>) {
        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.amount {
                let cr = stacks
                    .get_mut(instruction.from_stack - 1)
                    .unwrap()
                    .items
                    .pop()
                    .expect("Invalid instruction");

                stacks
                    .get_mut(instruction.to_stack - 1)
                    .unwrap()
                    .items
                    .push(cr);
            }
        })
    }
}

struct CraneNewModel {}

impl CraneStrategy for CraneNewModel {
    fn execute_instructions(&self, instructions: &Vec<Instruction>, stacks: &mut Vec<Stack>) {
        instructions.iter().for_each(|instruction| {
            let from_stack = stacks.get_mut(instruction.from_stack - 1).unwrap();
            let mut to_move: Vec<char> = from_stack
                .items
                .drain(from_stack.items.len() - instruction.amount as usize..)
                .collect();

            let to_stack = stacks.get_mut(instruction.to_stack - 1).unwrap();
            to_stack.items.append(&mut to_move);
        });
    }
}

impl FifthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Fifth Puzzle - Supply Stacks", "./inputs/5.txt"),
        }
    }

    fn get_storage(&self, crane_strategy: Box<dyn CraneStrategy>) -> Storage {
        Storage::new(self.get_stacks(), self.get_instructions(), crane_strategy)
    }

    fn get_stacks(&self) -> Vec<Stack> {
        let mut map_of_crates = HashMap::new();
        let re_crates = Regex::new(r".*\[[A-Z]\].*").unwrap();
        for cap in re_crates.captures_iter(&self.puzzle.input) {
            for (idx, c) in cap[0].chars().enumerate() {
                match c {
                    'A'..='Z' => {
                        let stack_as_str = map_of_crates.entry(idx).or_insert(String::new());
                        stack_as_str.push(c)
                    }
                    _ => continue,
                }
            }
        }
        let mut stacks = Vec::new();

        map_of_crates.iter().sorted().for_each(|(_, value)| {
            let mut crates = Vec::new();
            let mut value_clone = value.clone();
            loop {
                let crate_as_char = value_clone.pop();
                match crate_as_char {
                    Some(c) => crates.push(c),
                    None => break,
                }
            }
            stacks.push(Stack::new(crates));
        });
        stacks
    }

    fn get_instructions(&self) -> Vec<Instruction> {
        let re_moves = Regex::new(r".*move (\d+) from (\d+) to (\d+).*").unwrap();
        let mut instructions = Vec::new();

        for cap_move in re_moves.captures_iter(&self.puzzle.input) {
            instructions.push(Instruction::new(
                *&cap_move[1].parse().unwrap(),
                *&cap_move[2].parse().unwrap(),
                *&cap_move[3].parse().unwrap(),
            ));
        }
        instructions
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;
    #[test]
    fn crates_on_top_of_stacks_old_model() {
        assert_eq!(
            String::from("CMZ"),
            FifthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .get_storage(Box::new(CraneOldModel {}))
            .crates_on_top_of_stacks()
        )
    }

    #[test]
    fn crates_on_top_of_stacks_new_model() {
        assert_eq!(
            String::from("MCD"),
            FifthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .get_storage(Box::new(CraneNewModel {}))
            .crates_on_top_of_stacks()
        )
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "
                    [D]    
                [N] [C]    
                [Z] [M] [P]
                 1   2   3 
                
                move 1 from 2 to 1
                move 3 from 1 to 3
                move 2 from 2 to 1
                move 1 from 1 to 2",
            ),
        }
    }
}
