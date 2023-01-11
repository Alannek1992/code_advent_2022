use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct EleventhPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for EleventhPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.get_jungle().monkey_business(20), 0);
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

    fn play_the_game(&mut self, rounds: u64) {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() as u8 {
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

    fn monkey_business(&mut self, rounds: u64) -> u64 {
        self.play_the_game(rounds);
        let mut monkey_activities: Vec<u64> = self
            .monkeys
            .values()
            .map(|m| m.inspect_count)
            .sorted()
            .collect();

        monkey_activities.pop().unwrap() * monkey_activities.pop().unwrap()
    }
}

struct Monkey {
    items: Vec<Vec<u8>>,
    operation: Operation,
    test: TestDivisable,
    inspect_count: u64,
}

impl Monkey {
    fn new(items: Vec<u8>, operation: Operation, test: TestDivisable) -> Self {
        let mut items_as_digits = Vec::new();
        items
            .iter()
            .for_each(|x| items_as_digits.push(Operation::convert_num_to_digits(*x)));
        Self {
            items: items_as_digits,
            operation,
            test,
            inspect_count: 0,
        }
    }

    fn throw_to_another_monkey(&mut self) -> Option<(Vec<u8>, u8)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);
        let item = self.operation.execute(item);

        let receiver = if TestDivisable::is_divisable(&item, self.test.divisor) {
            self.test.success_receiver
        } else {
            self.test.fail_receiver
        };
        self.inspect_count += 1;
        Some((item, receiver))
    }

    fn catch_new_item(&mut self, item: Vec<u8>) {
        self.items.push(item);
    }
}

enum Operation {
    Plus(Vec<u8>),
    Multiply(Vec<u8>),
    Square,
}

impl Operation {
    fn convert_num_to_digits(number: u8) -> Vec<u8> {
        let mut item_as_digits = Vec::new();
        let mut x = number;

        loop {
            item_as_digits.push(x % 10);
            x /= 10;

            if x == 0 {
                break;
            }
        }
        item_as_digits.reverse();
        item_as_digits
    }

    fn execute(&mut self, input: Vec<u8>) -> Vec<u8> {
        match self {
            Operation::Plus(digits) => {
                let mut result = Vec::new();
                digits.reverse();
                let mut cloned_input = input.clone();
                cloned_input.reverse();
                let mut remainder = 0;
                for (idx, digit) in digits.iter().enumerate() {
                    let existing_digit = cloned_input.get(idx);
                    match existing_digit {
                        Some(d) => {
                            let sum = digit + d + remainder;
                            let correct_digit = sum % 10;
                            remainder = sum / 10;
                            result.push(correct_digit);
                        }
                        None => {
                            result.push(digit + remainder);
                            remainder = 0;
                        }
                    }
                }
                result.reverse();
                result
            }
            Operation::Multiply(digits) => {
                digits.reverse();
                let mut cloned_input = input.clone();
                cloned_input.reverse();
                let mut semi_results = Vec::new();
                for digit in digits.iter() {
                    let mut semi_result = Vec::new();
                    let mut remainder = 0;

                    for existing_digit in cloned_input.iter() {
                        let sum = digit * existing_digit + remainder;
                        let correct_digit = sum % 10;
                        remainder = sum / 10;
                        semi_result.push(correct_digit);
                    }
                    if remainder > 0 {
                        semi_result.push(remainder);
                    }
                    semi_result.reverse();
                    semi_results.push(semi_result);
                }
                let result = semi_results
                    .into_iter()
                    .reduce(|a, b| Operation::Plus(a).execute(b))
                    .unwrap();

                result
            }
            Operation::Square => Operation::Multiply(input.clone()).execute(input),
        }
    }
}

struct TestDivisable {
    divisor: u8,
    success_receiver: u8,
    fail_receiver: u8,
}

impl TestDivisable {
    fn new(divisor: u8, success_receiver: u8, fail_receiver: u8) -> Self {
        Self {
            divisor,
            success_receiver,
            fail_receiver,
        }
    }

    fn is_divisable(number_as_digits: &Vec<u8>, divisor: u8) -> bool {
        let mut remainder = 0;
        for digit in number_as_digits.iter() {
            remainder = (digit + remainder) % divisor;
        }
        remainder == 0
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
                .for_each(|m| items.push(*&m[0].parse::<u8>().unwrap()));

            let operation_captured = re_operation.captures(desc).unwrap();
            let operator = &operation_captured[1].chars().next().unwrap();
            let operation_value = &operation_captured[2];
            let operation = match operator {
                '+' => match operation_value.parse::<u8>() {
                    Ok(n) => Operation::Plus(Operation::convert_num_to_digits(n)),
                    Err(_) => unreachable!(),
                },
                '*' => match operation_value.parse::<u8>() {
                    Ok(n) => Operation::Multiply(Operation::convert_num_to_digits(n)),
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
            .monkey_business(10000)
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
