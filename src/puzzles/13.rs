use std::str::Chars;

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

#[derive(Debug)]
enum Packet {
    Number(u8),
    List(Vec<Self>),
}

impl Packet {
    fn add(&mut self, value: Self) {
        match self {
            Self::List(l) => l.push(value),
            _ => panic!("Adding value to a number packet"),
        }
    }
}

trait Comparison {
    fn compare(&self, another_packet: &Packet) -> ComparisonKind;
}

/*impl Comparison for Packet {
    fn compare(&self, another_packet: &Packet) -> ComparisonKind {}
}*/

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
        println!("{:?}", packets);
        /*let mut sum_of_pairs_in_right_order = 0;

        for (idx, (left_side, right_side)) in packets.iter().enumerate() {
            let mut accessor = 0;

            loop {
                let left_side_values = match left_side.get(accessor) {
                    Some(n) => n,
                    None => {
                        sum_of_pairs_in_right_order += idx + 1;
                        break;
                    }
                };
                let right_side_values = match right_side.get(accessor) {
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
        sum_of_pairs_in_right_order as u32*/
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
                let first_chunk_iter = &mut chunk[0].chars();
                first_chunk_iter.next();
                first_chunk_iter.next_back();

                let second_chunk_iter = &mut chunk[1].chars();
                second_chunk_iter.next();
                second_chunk_iter.next_back();
                (
                    self.read_packet(first_chunk_iter),
                    self.read_packet(second_chunk_iter),
                )
            })
            .collect()
    }

    fn read_packet(&self, iterator: &mut Chars) -> Packet {
        let mut packet = Packet::List(Vec::new());
        let mut acc_number = String::new();
        loop {
            let c = match iterator.next() {
                Some(c) => c,
                None => break,
            };
            match c {
                '[' => {
                    packet.add(self.read_packet(iterator));
                }
                ']' => {
                    if !acc_number.is_empty() {
                        packet.add(Packet::Number(acc_number.parse().unwrap()));
                    }
                    return packet;
                }
                ',' => {
                    if !acc_number.is_empty() {
                        packet.add(Packet::Number(acc_number.parse().unwrap()));
                        acc_number.clear();
                    }
                }
                _ => acc_number.push(c),
            };
        }
        if !acc_number.is_empty() {
            packet.add(Packet::Number(acc_number.parse().unwrap()));
        }
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
