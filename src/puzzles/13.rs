use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct ThirteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for ThirteenthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.sum_of_pairs_in_right_order(), 0);
    }
}

type PacketPair = (Packet, Packet);
type Packet = Vec<PacketValue>;
type PacketValue = Vec<u8>;

enum ComparisonKind {
    Smaller,
    Greater,
    Equal,
}

impl ThirteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Thirteenth Puzzle - Distress Signal", "./inputs/13.txt"),
        }
    }

    fn sum_of_pairs_in_right_order(&self) -> u32 {
        let packets = self.read_packets();
        /*let mut sum_of_pairs_in_right_order = 0;

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
            }*/
        10
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
        let mut strs = Vec::new();
        let mut acc_str = String::new();
        let mut previous_char = '*';
        let empty_list_code = ",255,";

        for c in input.chars() {
            match c {
                '[' => {
                    strs.push(acc_str.clone());
                    acc_str.clear();
                }
                ']' => {
                    if previous_char == '[' {
                        acc_str.push_str(empty_list_code);
                        continue;
                    }
                    strs.push(acc_str.clone());
                    acc_str.clear();
                }
                _ => acc_str.push(c),
            };
            previous_char = c;
        }

        if !acc_str.is_empty() {
            strs.push(acc_str.clone());
        }

        let mut packet = Packet::new();
        let re_number = Regex::new(r"\d+").unwrap();

        for finding in strs {
            let mut numbers = Vec::new();
            for capture in re_number.captures_iter(&finding) {
                let number = capture[0].parse::<u8>().unwrap();
                numbers.push(number);
            }
            if !numbers.is_empty() {
                packet.push(
                    numbers
                        .into_iter()
                        .filter(|number| *number != 255)
                        .collect(),
                );
            }
        }

        println!("{:?}", packet);

        packet
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn part_one() {
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
                [1,[2,[3,[4,[5,6,0]]]],8,9]
                
                [[1,[0,3,5,[2,1,3,3,5]],4,[[],5]],[],[0,[7,[5],7,7]]]
                [[[],[[],[5,2,8,9,7],1,5],[3,[]]]]",
            ),
        }
    }
}
