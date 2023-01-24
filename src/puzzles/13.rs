use regex::Regex;

use crate::PuzzleInfo;

pub struct ThirteenthPuzzle {
    puzzle: PuzzleInfo,
}

enum Packet {
    Number(u8),
    List(Vec<Self>),
}

type PacketPair = (Packet, Packet);

impl ThirteenthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Thirteenth Puzzle - Distress Signal", "./inputs/13.txt"),
        }
    }

    fn read_packets(&self) {
        let input: Vec<&str> = self
            .puzzle
            .input
            .lines()
            .map(|text| text.trim())
            .filter(|text| !text.is_empty())
            .collect();
        let re_packet = Regex::new(r".*\[(\d+)\].*").unwrap();

        for chunk in input.chunks(2) {
            re_packet.captures_iter(&chunk[0]).for_each(|capture| println!("{:?}", capture));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn part_one() {
        ThirteenthPuzzle {
            puzzle: get_puzzle_info(),
        }
        .read_packets();
        assert!(false);
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
