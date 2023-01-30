use std::{cmp::Ordering, str::Chars};

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct ThirteenthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for ThirteenthPuzzle {
    fn solution(&self) {
        print_solution(&self.puzzle.name, self.sum_of_pairs_in_right_order(), self.decoder_key());
    }
}

type PacketPair = (Packet, Packet);

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl PartialOrd for Packet {
    fn partial_cmp(&self, another_packet: &Packet) -> Option<Ordering> {
        match (self, another_packet) {
            (Packet::Number(n), Packet::Number(m)) => n.partial_cmp(m),
            (Packet::Number(n), packet) => {
                Packet::List(vec![Packet::Number(*n)]).partial_cmp(packet)
            }
            (packet, Packet::Number(n)) => {
                packet.partial_cmp(&Packet::List(vec![Packet::Number(*n)]))
            }
            (Packet::List(left), Packet::List(right)) => {
                let mut result = left.len().partial_cmp(&right.len());
                for (n, m) in left.iter().zip(right) {
                    if n.partial_cmp(m).unwrap() != Ordering::Equal {
                        result = n.partial_cmp(m);
                        break;
                    }
                }
                result
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
            if left_side < right_side {
                sum_of_pairs_in_right_order += idx + 1;
            }
        }
        sum_of_pairs_in_right_order as u32
    }

    fn decoder_key(&self) -> u32 {
        let mut sorted_packets = Vec::new();
        self.read_packets().into_iter().for_each(|(left, right)| {
            sorted_packets.push(left);
            sorted_packets.push(right);
        });
        let dividers = [Packet::List(vec![Packet::Number(2)]),Packet::List(vec![Packet::Number(6)])];
        sorted_packets.extend_from_slice(&dividers);
        sorted_packets.sort();

        let mut sum_of_pairs_in_right_order = 0;

        for (idx, p) in sorted_packets.iter().enumerate() {
            if dividers[0].cmp(p) == Ordering::Equal {
                sum_of_pairs_in_right_order += idx + 1;
            }

            if dividers[1].cmp(p) == Ordering::Equal {
                sum_of_pairs_in_right_order *= idx + 1;
                break;
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
    fn sum_of_pairs_in_right_order() {
        assert_eq!(
            13,
            ThirteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .sum_of_pairs_in_right_order()
        );
    }

    #[test]
    fn decoder_key() {
        assert_eq!(
            140,
            ThirteenthPuzzle {
                puzzle: get_puzzle_info(),
            }
            .decoder_key()
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
