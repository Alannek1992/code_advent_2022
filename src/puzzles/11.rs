use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct EleventhPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for EleventhPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.monkey_business(20), 0);
    }
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

    fn play_the_game(&mut self, rounds: i32) {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() as i32 {
                loop {
                    let monkey = match self.monkeys.get_mut(&i) {
                        Some(m) => m,
                        None => break,
                    };
                    let (item, receiver) = match monkey.throw_to_another_monkey() {
                        Some(t) => t,
                        None => break,
                    };

                    let monkey_receiver = self.monkeys.get_mut(&receiver).unwrap();
                    monkey_receiver.catch_new_item(item);
                }
            }
        }
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: TestDivisable,
    inspect_count: i32,
}

impl Monkey {
    fn new(items: Vec<i32>, operation: Operation, test: TestDivisable) -> Self {
        Self {
            items,
            operation,
            test,
            inspect_count: 0,
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
        self.inspect_count += 1;
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

impl TestDivisable {
    fn new(divider: i32, success_receiver: i32, fail_receiver: i32) -> Self {
        Self {
            divider,
            success_receiver,
            fail_receiver,
        }
    }
}

impl EleventhPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Eleventh Puzzle - Monkey in the Middle", "./inputs/11.txt"),
        }
    }

    fn monkey_business(&self, rounds: i32) -> i32 {
        let mut jungle = Jungle::new(self.get_monkeys());
        jungle.play_the_game(rounds);
        let mut monkey_activities: Vec<i32> = jungle
            .monkeys
            .into_values()
            .map(|m| m.inspect_count)
            .sorted()
            .collect();

        monkey_activities.pop().unwrap() * monkey_activities.pop().unwrap()
    }

    fn get_monkeys(&self) -> Vec<Monkey> {
        let mut monkeys = Vec::new();
        let descriptions: Vec<&str> = self
            .puzzle
            .input
            .split("Monkey")
            .filter(|e| !e.is_empty())
            .collect();
        let re_items = Regex::new(r".*Starting items:.*\n").unwrap();
        let re_item_no = Regex::new(r"\d+").unwrap();
        let re_operation = Regex::new(r".*old ([+*]) (old|\d+)").unwrap();
        let re_test_divider = Regex::new(r".*by (\d+)").unwrap();
        let re_test_true = Regex::new(r".*true.* (\d+)").unwrap();
        let re_test_false = Regex::new(r".*false.* (\d+)").unwrap();

        for desc in descriptions.iter() {
            let mut items = Vec::new();
            let extracted_items_as_str = &re_items.captures(desc).unwrap()[0];
            re_item_no
                .captures_iter(extracted_items_as_str)
                .for_each(|m| items.push(*&m[0].parse::<i32>().unwrap()));

            let operation_captured = re_operation.captures(desc).unwrap();
            let operator = &operation_captured[1].chars().next().unwrap();
            let operation_value = &operation_captured[2];
            let operation = match operator {
                '+' => match operation_value.parse::<i32>() {
                    Ok(n) => Operation::Plus(n),
                    Err(_) => unreachable!(),
                },
                '*' => match operation_value.parse::<i32>() {
                    Ok(n) => Operation::Multiply(n),
                    Err(_) => Operation::Power,
                },
                _ => unreachable!(),
            };

            let test_divider_captured = re_test_divider.captures(desc).unwrap();
            let test_true_captured = re_test_true.captures(desc).unwrap();
            let test_false_captured = re_test_false.captures(desc).unwrap();
            let test = TestDivisable::new(
                *&test_divider_captured[1].parse().unwrap(),
                *&test_true_captured[1].parse().unwrap(),
                *&test_false_captured[1].parse().unwrap(),
            );

            monkeys.push(Monkey::new(items, operation, test));
        }

        monkeys
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn monkey_business() {
        assert_eq!(
            10605,
            EleventhPuzzle {
                puzzle: get_puzzle_info(),
            }
            .monkey_business(20)
        );
    }

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
