use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct EleventhPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for EleventhPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.get_jungle()
                .monkey_business(20, CompressionKind::DivisonByThree),
            self.get_jungle()
                .monkey_business(10000, CompressionKind::ProductOfDivisors),
        );
    }
}

struct Jungle {
    monkeys: HashMap<u8, Monkey>,
}

impl Jungle {
    fn new(monkeys: Vec<Monkey>) -> Self {
        let mut monkeys_as_map = HashMap::new();
        for (idx, monkey) in monkeys.into_iter().enumerate() {
            monkeys_as_map.insert(idx as u8, monkey);
        }
        Self {
            monkeys: monkeys_as_map,
        }
    }

    fn monkey_business(&mut self, rounds: u64, compression_kind: CompressionKind) -> u64 {
        let compressor = match compression_kind {
            CompressionKind::DivisonByThree => 3,
            CompressionKind::ProductOfDivisors => {
                self.monkeys.values().map(|m| m.test.divisor).product()
            }
        };
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() as u8 {
                loop {
                    let monkey = match self.monkeys.get_mut(&i) {
                        Some(m) => m,
                        None => break,
                    };
                    let (item, receiver) =
                        match monkey.throw_to_another_monkey(compressor, &compression_kind) {
                            Some(t) => t,
                            None => break,
                        };

                    let monkey_receiver = self.monkeys.get_mut(&receiver).unwrap();
                    monkey_receiver.catch_new_item(item);
                }
            }
        }
        let mut monkey_activities: Vec<u64> = self
            .monkeys
            .values()
            .map(|m| m.inspect_count)
            .sorted()
            .collect();

        monkey_activities.pop().unwrap() * monkey_activities.pop().unwrap()
    }
}

enum CompressionKind {
    DivisonByThree,
    ProductOfDivisors,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: TestDivisable,
    inspect_count: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, test: TestDivisable) -> Self {
        Self {
            items,
            operation,
            test,
            inspect_count: 0,
        }
    }

    fn throw_to_another_monkey(
        &mut self,
        compression: u64,
        compression_kind: &CompressionKind,
    ) -> Option<(u64, u8)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);
        let item = self.operation.execute(item);
        let item = match compression_kind {
            CompressionKind::DivisonByThree => item / compression,
            CompressionKind::ProductOfDivisors => item % compression,
        };

        let receiver = if item % self.test.divisor == 0 {
            self.test.success_receiver
        } else {
            self.test.fail_receiver
        };
        self.inspect_count += 1;
        Some((item, receiver))
    }

    fn catch_new_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

enum Operation {
    Plus(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn execute(&self, input: u64) -> u64 {
        match self {
            Operation::Plus(n) => n + input,
            Operation::Multiply(n) => n * input,
            Operation::Square => input * input,
        }
    }
}

struct TestDivisable {
    divisor: u64,
    success_receiver: u8,
    fail_receiver: u8,
}

impl TestDivisable {
    fn new(divisor: u64, success_receiver: u8, fail_receiver: u8) -> Self {
        Self {
            divisor,
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

    fn get_jungle(&self) -> Jungle {
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
        let re_test_divisor = Regex::new(r".*by (\d+)").unwrap();
        let re_test_true = Regex::new(r".*true.* (\d+)").unwrap();
        let re_test_false = Regex::new(r".*false.* (\d+)").unwrap();

        for desc in descriptions.iter() {
            let mut items = Vec::new();
            let extracted_items_as_str = &re_items.captures(desc).unwrap()[0];
            re_item_no
                .captures_iter(extracted_items_as_str)
                .for_each(|m| items.push(*&m[0].parse::<u64>().unwrap()));

            let operation_captured = re_operation.captures(desc).unwrap();
            let operator = &operation_captured[1].chars().next().unwrap();
            let operation_value = &operation_captured[2];
            let operation = match operator {
                '+' => match operation_value.parse::<u64>() {
                    Ok(n) => Operation::Plus(n),
                    Err(_) => unreachable!(),
                },
                '*' => match operation_value.parse::<u64>() {
                    Ok(n) => Operation::Multiply(n),
                    Err(_) => Operation::Square,
                },
                _ => unreachable!(),
            };

            let test_divisor_captured = re_test_divisor.captures(desc).unwrap();
            let test_true_captured = re_test_true.captures(desc).unwrap();
            let test_false_captured = re_test_false.captures(desc).unwrap();
            let test = TestDivisable::new(
                *&test_divisor_captured[1].parse().unwrap(),
                *&test_true_captured[1].parse().unwrap(),
                *&test_false_captured[1].parse().unwrap(),
            );

            monkeys.push(Monkey::new(items, operation, test));
        }

        Jungle::new(monkeys)
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn monkey_business_high_load() {
        assert_eq!(
            2713310158,
            EleventhPuzzle {
                puzzle: get_puzzle_info(),
            }
            .get_jungle()
            .monkey_business(10000, CompressionKind::ProductOfDivisors)
        );
    }

    #[test]
    fn monkey_business() {
        assert_eq!(
            10605,
            EleventhPuzzle {
                puzzle: get_puzzle_info(),
            }
            .get_jungle()
            .monkey_business(20, CompressionKind::DivisonByThree)
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
