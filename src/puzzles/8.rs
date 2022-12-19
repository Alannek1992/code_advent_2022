use std::{collections::HashMap};

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct EighthPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for EighthPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.get_forest().visible_trees_from_outside(),
            self.get_forest().highest_scenic_score_for_any_tree(),
        );
    }
}

struct Forest {
    trees: HashMap<TreeCoordinate, TreeHeight>,
    total_lines: u32,
    total_cols: u32,
    visibility_direction_variants: [VisibilityDirection; 4],
}

impl Forest {
    fn new(trees: Vec<Tree>, total_lines: u32, total_cols: u32) -> Self {
        let mut trees_as_map = HashMap::new();
        trees.iter().for_each(|tree| {
            trees_as_map.insert(tree.coordinate, tree.height);
        });

        Self {
            trees: trees_as_map,
            total_lines,
            total_cols,
            visibility_direction_variants: [
                VisibilityDirection::TOP,
                VisibilityDirection::RIGHT,
                VisibilityDirection::BOTTOM,
                VisibilityDirection::LEFT,
            ],
        }
    }

    fn visible_trees_from_outside(&self) -> u32 {
        let mut visible_trees = 0;
        for (coordinate, height) in self.trees.iter() {
            if self.is_edge(*coordinate) {
                visible_trees += 1;
                continue;
            }

            for variant in self.visibility_direction_variants.iter() {
                if self.is_visible(*coordinate, *height, variant).0 {
                    visible_trees += 1;
                    break;
                }
            }
        }
        visible_trees
    }

    fn highest_scenic_score_for_any_tree(&self) -> u32 {
        let mut highest_scenic_score = 1;
        for (coordinate, height) in self.trees.iter() {
            if self.is_edge(*coordinate) {
                continue;
            }

            let mut scenic_score = 1;

            for variant in self.visibility_direction_variants.iter() {
                scenic_score *= self.is_visible(*coordinate, *height, variant).1
            }

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
        highest_scenic_score
    }

    fn is_edge(&self, coordinate: TreeCoordinate) -> bool {
        (coordinate.0 == 0 || coordinate.1 == 0)
            || (coordinate.0 == self.total_lines || coordinate.1 == self.total_cols)
    }

    fn is_visible(
        &self,
        coordinate: TreeCoordinate,
        height: u32,
        from_direction: &VisibilityDirection,
    ) -> (bool, u32) {
        let mut trees_in_view = 0;
        let sequences: Vec<u32> = match from_direction {
            VisibilityDirection::TOP => (0..coordinate.0).rev().step_by(1).collect(),
            VisibilityDirection::RIGHT => (coordinate.1 + 1..=self.total_cols).step_by(1).collect(),
            VisibilityDirection::BOTTOM => (coordinate.0 + 1..=self.total_lines).step_by(1).collect(),
            VisibilityDirection::LEFT => (0..coordinate.1).rev().step_by(1).collect(),
        };

        for i in sequences {
            let upfront_tree = match from_direction {
                VisibilityDirection::TOP | VisibilityDirection::BOTTOM => {
                    self.trees.get(&(i, coordinate.1))
                }
                VisibilityDirection::LEFT | VisibilityDirection::RIGHT => {
                    self.trees.get(&(coordinate.0, i))
                }
            };

            trees_in_view += 1;

            if *upfront_tree.unwrap() >= height {
                return (false, trees_in_view);
            }
        }

        (true, trees_in_view)
    }
}

enum VisibilityDirection {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

type TreeCoordinate = (u32, u32);
type TreeHeight = u32;
struct Tree {
    height: TreeHeight,
    coordinate: TreeCoordinate,
}

impl Tree {
    fn new(line: u32, col: u32, height: u32) -> Self {
        Self {
            coordinate: (line, col),
            height,
        }
    }
}

impl EighthPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Eighth Puzzle - Treetop Tree House", "./inputs/8.txt"),
        }
    }

    fn get_forest(&self) -> Forest {
        let mut trees: Vec<Tree> = Vec::new();
        let mut line_no = 0;

        for line in self.puzzle.input.lines() {
            for (col, c) in line.trim().chars().enumerate() {
                trees.push(Tree::new(line_no, col as u32, c.to_digit(10).unwrap()));
            }
            line_no += 1;
        }

        Forest::new(
            trees,
            line_no - 1,
            (self.puzzle.input.lines().next().unwrap().len() - 1) as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn highest_scenic_score_for_any_tree() {
        assert_eq!(
            8,
            EighthPuzzle {
                puzzle: get_puzzle_info()
            }
            .get_forest()
            .highest_scenic_score_for_any_tree()
        )
    }

    #[test]
    fn visible_trees_from_outside() {
        assert_eq!(
            21,
            EighthPuzzle {
                puzzle: get_puzzle_info()
            }
            .get_forest()
            .visible_trees_from_outside()
        )
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "30373
                25512
                65332
                33549
                35390",
            ),
        }
    }
}
