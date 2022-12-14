use core::panic;

use regex::Regex;

use crate::PuzzleInfo;

pub struct SeventhPuzzle {
    puzzle: PuzzleInfo,
}

struct FileSystem<'t> {
    root_node: FileSystemNode<'t>,
}

impl<'t> FileSystem<'t> {
    fn new() -> Self {
        Self {
            root_node: FileSystemNode::Directory(None, Vec::new()),
        }
    }

    fn build_file_system_from_line_commands(line_commands: Vec<LineCommand>) -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum LineCommand {
    CD(CDKind),
    LS(Vec<LSOutput>),
}

#[derive(Debug)]
enum CDKind {
    Root,
    Up,
    Down(char),
}

#[derive(Debug)]
enum LSOutput {
    File(i32, String),
    Directory(char),
}

#[derive(Debug)]
enum FileSystemNode<'t> {
    Directory(Option<&'t Self>, Vec<Self>),
    File(&'t Self, i32),
}

impl<'t> FileSystemNode<'t> {
    pub fn append_node(&mut self, node: FileSystemNode<'t>) {
        match self {
            FileSystemNode::Directory(_, child_nodes) => child_nodes.push(node),
            FileSystemNode::File(_, _) => panic!("Cannot append node to a file!"),
        }
    }

    pub fn parent(&self) -> &Self {
        match self {
            FileSystemNode::Directory(parent, _) => {
                parent.expect("Should not be accessing out of root")
            }
            FileSystemNode::File(parent, _) => parent,
        }
    }

    pub fn childs(&self) -> &Vec<Self> {
        match self {
            FileSystemNode::Directory(_, childs) => childs,
            FileSystemNode::File(parent, _) => panic!("File cannot have childs"),
        }
    }
}

impl SeventhPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Seventh Puzzle - No Space Left On Device", "./inputs/7.txt"),
        }
    }

    fn get_line_commands(&self) -> Vec<LineCommand> {
        let re_command = Regex::new(r"\$.*").unwrap();
        let re_cd_root = Regex::new(r".*cd /").unwrap();
        let re_cd_down = Regex::new(r".*cd [a-z]").unwrap();
        let re_cd_up = Regex::new(r".*cd ..").unwrap();
        let re_file = Regex::new(r"(\d+) (.*)").unwrap();
        let re_dir = Regex::new(r".*([a-z])").unwrap();

        let mut line_commads = Vec::new();
        let mut file_system_nodes = Vec::new();

        for line in self.puzzle.input.lines() {
            let line = line.trim();

            if re_command.is_match(line) {
                if !file_system_nodes.is_empty() {
                    line_commads.push(LineCommand::LS(file_system_nodes.drain(..).collect()))
                }

                if re_cd_root.is_match(line) {
                    line_commads.push(LineCommand::CD(CDKind::Root));
                } else if re_cd_down.is_match(line) {
                    line_commads.push(LineCommand::CD(CDKind::Down('a')));
                } else if re_cd_up.is_match(line) {
                    line_commads.push(LineCommand::CD(CDKind::Up));
                }

                continue;
            }

            if let Some(file) = re_file.captures(line) {
                file_system_nodes.push(LSOutput::File(
                    *&file[1].parse().unwrap(),
                    String::from(&file[2]),
                ))
            } else {
                let test = re_dir.captures(line).unwrap();
                file_system_nodes.push(LSOutput::Directory(*&test[1].chars().next().unwrap()))
            }
        }

        if !file_system_nodes.is_empty() {
            line_commads.push(LineCommand::LS(file_system_nodes))
        }

        println!("{:?}", line_commads);

        line_commads
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn part_one() {
        SeventhPuzzle {
            puzzle: get_puzzle_info(),
        }
        .get_line_commands();
        assert!(false);
    }

    fn get_puzzle_info() -> PuzzleInfo {
        PuzzleInfo {
            name: String::from("Test"),
            input: String::from(
                "$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k",
            ),
        }
    }
}
