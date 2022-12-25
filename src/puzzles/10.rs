use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct TenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for TenthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.get_handheld().sum_of_signal_strenghts().to_string(),
            String::from("FJUBULRZ"),
        );

        self.get_handheld().render();
    }
}

struct Hanheld {
    cpu: CPU,
    crt: CRT,
    instructions: Vec<Instruction>,
}

impl Hanheld {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cpu: CPU::new(),
            crt: CRT::new(),
            instructions,
        }
    }

    fn sum_of_signal_strenghts(&self) -> i32 {
        self.cpu
            .execute_instructions(&self.instructions)
            .iter()
            .sum()
    }

    fn render(&self) {
        self.crt.execute_instructions(&self.instructions);
    }
}

struct CPU {
    cycles_to_capture: Vec<i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            cycles_to_capture: vec![20, 60, 100, 140, 180, 220],
        }
    }

    fn execute_instructions(&self, instructions: &Vec<Instruction>) -> Vec<i32> {
        let mut captured_signal_strengths = Vec::new();
        let mut x = 1;
        let mut total_iterations: i32 = 0;

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.get_cycle_length() {
                total_iterations += 1;
                if self.cycles_to_capture.contains(&total_iterations) {
                    captured_signal_strengths.push(x * total_iterations);
                }
            }
            match instruction {
                Instruction::AddX(n) => x += n,
                _ => {}
            }
        });

        captured_signal_strengths
    }
}

struct CRT {
    display_height: i32,
    display_width: i32,
    sprite_length: i32,
}

impl CRT {
    fn new() -> Self {
        Self {
            display_height: 6,
            display_width: 40,
            sprite_length: 2,
        }
    }

    fn execute_instructions(&self, instructions: &Vec<Instruction>) {
        let mut col_idx: i32 = 0;
        let mut row_idx = 0;
        let mut row = String::new();

        let mut x = 1;

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.get_cycle_length() {
                if col_idx == self.display_width {
                    col_idx = 0;
                    row_idx += 1;
                    println!("{row}");
                    row = String::new();
                }

                if (x - col_idx).abs() < self.sprite_length {
                    row.push('#')
                } else {
                    row.push('.')
                }

                col_idx += 1;

                if row_idx == self.display_height - 1 && col_idx == self.display_width {
                    println!("{row}");
                }
            }
            match instruction {
                Instruction::AddX(n) => x += n,
                _ => {}
            }
        });
    }
}

enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    fn get_cycle_length(&self) -> u32 {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

impl TenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Tenth Puzzle - Cathode-Ray Tube", "./inputs/10.txt"),
        }
    }

    fn get_handheld(&self) -> Hanheld {
        let mut instructions = Vec::new();
        let re_addx = Regex::new(r".* (-?\d+)").unwrap();

        for line in self.puzzle.input.lines() {
            let line = line.trim();

            if re_addx.is_match(line) {
                let r_capture = re_addx.captures(line).unwrap();
                instructions.push(Instruction::AddX(*&r_capture[1].parse().unwrap()))
            } else {
                instructions.push(Instruction::Noop)
            }
        }

        Hanheld::new(instructions)
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn crt_render() {
        TenthPuzzle {
            puzzle: get_puzzle_info(),
        }
        .get_handheld()
        .render();
        assert!(true);
    }

    #[test]
    fn sum_of_signal_strenghts() {
        assert_eq!(
            13140,
            TenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .get_handheld()
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
                noop",
            ),
        }
    }
}
