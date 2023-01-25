use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct ThirteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for ThirteenthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.sum_of_pairs_in_right_order(), 0);
    }
}

#[derive(Debug)]
struct Packet {
    values: Vec<PacketValue>,
}

impl Packet {
    fn new(mut values: Vec<PacketValue>) -> Self {
        values.sort_by(|a, b| a.order.cmp(&b.order));
        Self { values }
    }
}

#[derive(Debug)]
struct PacketValue {
    data: PacketValueKind,
    order: u32,
}

impl PacketValue {
    fn new(data: PacketValueKind, order: u32) -> Self {
        Self { data, order }
    }

    fn compare(&self, another_packet_value: &PacketValue) -> ComparisonKind {
        let data = match &self.data {
            PacketValueKind::List(list) => list.clone(),
            PacketValueKind::Number(number) => vec![*number],
        };

        let another_packet_value_data = match &another_packet_value.data {
            PacketValueKind::List(list) => list.clone(),
            PacketValueKind::Number(number) => vec![*number],
        };

        for (idx, value) in data.iter().enumerate() {
            let another_value = match another_packet_value_data.get(idx) {
                Some(n) => n,
                None => return ComparisonKind::Greater,
            };

            if value > another_value {
                return ComparisonKind::Greater;
            }

            if value < another_value {
                return ComparisonKind::Smaller;
            }
        }

        if another_packet_value_data.len() > data.len() {
            return ComparisonKind::Smaller;
        }

        ComparisonKind::Equal
    }
}

enum ComparisonKind {
    Smaller,
    Greater,
    Equal,
}

#[derive(Debug)]
enum PacketValueKind {
    Number(u8),
    List(Vec<u8>),
}
impl PacketValueKind {
    fn add(&mut self, item: u8) {
        match self {
            PacketValueKind::List(l) => l.push(item),
            _ => unreachable!(),
        }
    }
}

type PacketPair = (Packet, Packet);

#[derive(Debug)]
struct Stack {
    items: Vec<PacketValue>,
}

impl Stack {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn push(&mut self, value: PacketValue) {
        self.items.push(value);
    }

    fn pop(&mut self) -> Option<PacketValue> {
        self.items.pop()
    }

    fn peek_mut(&mut self) -> Option<&mut PacketValue> {
        self.items.last_mut()
    }
}

impl ThirteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Thirteenth Puzzle - Distress Signal", "./inputs/13.txt"),
        }
    }

    fn sum_of_pairs_in_right_order(&self) -> u32 {
        let packets = self.read_packets();
        let mut sum_of_pairs_in_right_order = 0;

        for (idx, (left_side, right_side)) in packets.iter().enumerate() {
            let mut accessor = 0;

            loop {
                let left_side_values = match left_side.values.get(accessor) {
                    Some(n) => n,
                    None => {
                        sum_of_pairs_in_right_order += idx + 1;
                        break;
                    }
                };
                let right_side_values = match right_side.values.get(accessor) {
                    Some(n) => n,
                    None => break,
                };

                match left_side_values.compare(right_side_values) {
                    ComparisonKind::Smaller => {
                        sum_of_pairs_in_right_order += idx + 1;
                        break;
                    }
                    ComparisonKind::Greater => break,
                    ComparisonKind::Equal => accessor += 1,
                };
            }
        }

        sum_of_pairs_in_right_order as u32
    }

    fn read_packets(&self) -> Vec<PacketPair> {
        let input: Vec<&str> = self
            .puzzle
            .input
            .lines()
            .map(|text| text.trim())
            .filter(|text| !text.is_empty())
            .collect();

        input
            .chunks(2)
            .map(|chunk| {
                (
                    self.read_packet_data(&chunk[0]),
                    self.read_packet_data(&chunk[1]),
                )
            })
            .collect()
    }

    fn read_packet_data(&self, input: &str) -> Packet {
        let mut stack = Stack::new();
        let mut packet_values = Vec::new();
        let mut nested_level = 0;
        let mut order = 0;
        let mut acc_number = String::new();

        let mut iter = input.chars();
        iter.next();
        iter.next_back();

        for c in iter {
            match c {
                '[' => {
                    stack.push(PacketValue::new(PacketValueKind::List(Vec::new()), order));
                    order += 1;
                    nested_level += 1
                }
                ']' => {
                    let mut packet_value = stack.pop().unwrap();
                    if !acc_number.is_empty() {
                        packet_value.data.add(acc_number.parse().unwrap());
                        acc_number.clear();
                    }
                    packet_values.push(packet_value);
                    nested_level -= 1;
                }
                ',' => {
                    if !acc_number.is_empty() {
                        if nested_level == 0 {
                            packet_values.push(PacketValue::new(
                                PacketValueKind::Number(acc_number.parse().unwrap()),
                                order,
                            ));

                            acc_number.clear();
                            order += 1;
                        } else {
                            stack
                                .peek_mut()
                                .unwrap()
                                .data
                                .add(acc_number.parse().unwrap());
                            acc_number.clear();
                        }
                    }
                }
                _ => acc_number.push(c),
            }
        }

        if !acc_number.is_empty() {
            packet_values.push(PacketValue::new(
                PacketValueKind::Number(acc_number.parse().unwrap()),
                order,
            ));
        }

        Packet::new(packet_values)
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn sum_of_pairs_in_right_order() {
        assert_eq!(
            13,
            ThirteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sum_of_pairs_in_right_order()
        );
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "[1,1,3,1,1]
                [1,1,5,1,1]
                
                [[1],[2,3,4]]
                [[1],4]
                
                [9]
                [[8,7,6]]
                
                [[4,4],4,4]
                [[4,4],4,4,4]
                
                [7,7,7,7]
                [7,7,7]
                
                []
                [3]
                
                [[[]]]
                [[]]
                
                [1,[2,[3,[4,[5,6,7]]]],8,9]
                [1,[2,[3,[4,[5,6,0]]]],8,9]",
            ),
        }
    }
}
