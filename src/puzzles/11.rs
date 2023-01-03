use std::collections::VecDeque;

use crate::PuzzleInfo;

pub struct EleventhPuzzle {
    puzzle: PuzzleInfo,
}

enum Operation {
    Plus(i32),
    Multiply(i32),
    Power,
}

impl Operation {
    fn execute_operation(&self, input: i32) -> i32 {
        match self {
            Operation::Plus(n) => input + n,
            Operation::Multiply(n) => input * n,
            Operation::Power => input * input,
        }
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: fn(input: i32) -> i32,
}

impl Monkey {
    fn new(items: Vec<i32>, operation: fn(input: i32) -> i32, test: fn(input: i32) -> i32) -> Self {
        Self {
            items,
            operation,
            test,
        }
    }

    fn throw_to_another_monkey(&mut self) -> i32 {
        let item = self.items.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "Monkey 0:
                Starting items: 79, 98
                Operation: new = old * 19
                Test: divisible by 23
                  If true: throw to monkey 2
                  If false: throw to monkey 3
              
              Monkey 1:
                Starting items: 54, 65, 75, 74
                Operation: new = old + 6
                Test: divisible by 19
                  If true: throw to monkey 2
                  If false: throw to monkey 0
              
              Monkey 2:
                Starting items: 79, 60, 97
                Operation: new = old * old
                Test: divisible by 13
                  If true: throw to monkey 1
                  If false: throw to monkey 3
              
              Monkey 3:
                Starting items: 74
                Operation: new = old + 3
                Test: divisible by 17
                  If true: throw to monkey 0
                  If false: throw to monkey 1",
            ),
        }
    }
}
