use std::collections::{HashMap, VecDeque};

use crate::PuzzleInfo;

pub struct EleventhPuzzle {
    puzzle: PuzzleInfo,
}

struct Jungle {
    monkeys: HashMap<i32, Monkey>,
}

impl Jungle {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let mut monkeys_as_map = HashMap::new();
        for (idx, monkey) in monkeys.into_iter().enumerate() {
            monkeys_as_map.insert(idx as i32, monkey);
        }
        Self {
            monkeys: monkeys_as_map,
        }
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: TestDivisable,
}

impl Monkey {
    fn new(items: Vec<i32>, operation: Operation, test: TestDivisable) -> Self {
        Self {
            items,
            operation,
            test,
        }
    }

    fn throw_to_another_monkey(&mut self) -> Option<(i32, i32)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);
        let item = self.operation.execute(item) / 3;

        let remainder = item % self.test.divider;
        let receiver = if remainder == 0 {
            self.test.success_receiver
        } else {
            self.test.fail_receiver
        };
        Some((item, receiver))
    }

    fn catch_new_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

enum Operation {
    Plus(i32),
    Multiply(i32),
    Power,
}

impl Operation {
    fn execute(&self, input: i32) -> i32 {
        match self {
            Operation::Plus(n) => input + n,
            Operation::Multiply(n) => input * n,
            Operation::Power => input * input,
        }
    }
}

struct TestDivisable {
    divider: i32,
    success_receiver: i32,
    fail_receiver: i32,
}

impl EleventhPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Eleventh Puzzle - Monkey in the Middle", "./inputs/11.txt"),
        }
    }

    fn get_monkeys(&self) -> Vec<Monkey> {
        let mut monkeys = Vec::new();



        monkeys
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
