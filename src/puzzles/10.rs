use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct TenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for TenthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.sum_of_signal_strenghts(), 0);
    }
}

struct CPU {
    cycles_to_capture: Vec<i32>,
    instructions: Vec<CPUInstruction>,
}

impl CPU {
    fn new(cycles_to_capture: Vec<i32>, instructions: Vec<CPUInstruction>) -> Self {
        Self {
            cycles_to_capture,
            instructions,
        }
    }

    fn execute_instructions(&self) -> Vec<i32> {
        let mut captured_signal_strengths = Vec::new();
        let mut x = 1;
        let mut total_iterations: i32 = 0;

        self.instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.get_cycle_length() {
                total_iterations += 1;
                if self.cycles_to_capture.contains(&total_iterations) {
                    captured_signal_strengths.push(x * total_iterations);
                }
            }
            match instruction {
                CPUInstruction::AddX(n) => x += n,
                _ => {}
            }
        });

        captured_signal_strengths
    }
}

enum CPUInstruction {
    AddX(i32),
    Noop,
}

impl CPUInstruction {
    fn get_cycle_length(&self) -> u32 {
        match self {
            CPUInstruction::AddX(_) => 2,
            CPUInstruction::Noop => 1,
        }
    }
}

impl TenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Tenth Puzzle - Cathode-Ray Tube", "./inputs/10.txt"),
        }
    }

    fn sum_of_signal_strenghts(&self) -> i32 {
        let instructions = self.read_instructions();
        let cycles_to_capture = vec![20, 60, 100, 140, 180, 220];

        CPU::new(cycles_to_capture, instructions)
            .execute_instructions()
            .iter()
            .sum()
    }

    fn read_instructions(&self) -> Vec<CPUInstruction> {
        let mut instructions = Vec::new();
        let re_addx = Regex::new(r".* (-?\d+)").unwrap();

        for line in self.puzzle.input.lines() {
            let line = line.trim();

            if re_addx.is_match(line) {
                let r_capture = re_addx.captures(line).unwrap();
                instructions.push(CPUInstruction::AddX(*&r_capture[1].parse().unwrap()))
            } else {
                instructions.push(CPUInstruction::Noop)
            }
        }

        instructions
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn sum_of_signal_strenghts() {
        assert_eq!(
            13140,
            TenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sum_of_signal_strenghts()
        )
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "addx 15
                addx -11
                addx 6
                addx -3
                addx 5
                addx -1
                addx -8
                addx 13
                addx 4
                noop
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx -35
                addx 1
                addx 24
                addx -19
                addx 1
                addx 16
                addx -11
                noop
                noop
                addx 21
                addx -15
                noop
                noop
                addx -3
                addx 9
                addx 1
                addx -3
                addx 8
                addx 1
                addx 5
                noop
                noop
                noop
                noop
                noop
                addx -36
                noop
                addx 1
                addx 7
                noop
                noop
                noop
                addx 2
                addx 6
                noop
                noop
                noop
                noop
                noop
                addx 1
                noop
                noop
                addx 7
                addx 1
                noop
                addx -13
                addx 13
                addx 7
                noop
                addx 1
                addx -33
                noop
                noop
                noop
                addx 2
                noop
                noop
                noop
                addx 8
                noop
                addx -1
                addx 2
                addx 1
                noop
                addx 17
                addx -9
                addx 1
                addx 1
                addx -3
                addx 11
                noop
                noop
                addx 1
                noop
                addx 1
                noop
                noop
                addx -13
                addx -19
                addx 1
                addx 3
                addx 26
                addx -30
                addx 12
                addx -1
                addx 3
                addx 1
                noop
                noop
                noop
                addx -9
                addx 18
                addx 1
                addx 2
                noop
                noop
                addx 9
                noop
                noop
                noop
                addx -1
                addx 2
                addx -37
                addx 1
                addx 3
                noop
                addx 15
                addx -21
                addx 22
                addx -6
                addx 1
                noop
                addx 2
                addx 1
                noop
                addx -10
                noop
                noop
                addx 20
                addx 1
                addx 2
                addx 2
                addx -6
                addx -11
                noop
                noop
                noop
                ",
            ),
        }
    }
}
