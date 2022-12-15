use std::collections::HashMap;

use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct SeventhPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for SeventhPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.total_size_of_dirs_to_delete(10000),
            self.total_size_of_dirs_to_delete(10000),
        );
    }
}

#[derive(Debug)]
struct FileSystem {
    directories: HashMap<char, Directory>,
}

impl FileSystem {
    fn new() -> Self {
        let mut directories = HashMap::new();
        directories.insert('/', Directory::new(None, '/'));
        Self { directories }
    }

    fn build_file_system_from_line_commands(line_commands: Vec<LineCommand>) -> Self {
        let mut file_system = Self::new();
        let mut current_dir = '/';

        for line_command in line_commands {
            match line_command {
                LineCommand::CD(kind) => match kind {
                    CDKind::Root => {
                        current_dir = '/';
                    }
                    CDKind::Up => {
                        current_dir = file_system
                            .directories
                            .get(&current_dir)
                            .unwrap()
                            .parent_dir_name
                            .unwrap()
                    }
                    CDKind::Down(dir_name) => current_dir = dir_name,
                },
                LineCommand::LS(output) => {
                    for ls_out in output {
                        match ls_out {
                            LSOutput::File(size, name) => {
                                file_system
                                    .directories
                                    .get_mut(&current_dir)
                                    .unwrap()
                                    .add_file(File::new(name, size));
                            }
                            LSOutput::Directory(dir_name) => {
                                let new_dir = Directory::new(Some(current_dir), dir_name);
                                let current_dir_ref =
                                    file_system.directories.get_mut(&current_dir).unwrap();
                                current_dir_ref.add_sub_dir(new_dir.name);
                                file_system.directories.insert(dir_name, new_dir);
                            }
                        };
                    }
                }
            }
        }

        file_system
    }

    fn calculate_dir_size(&self, dir_name: char) -> i32 {
        let dir = self.directories.get(&dir_name).expect("Dir does not exist");
        let mut curr_size = dir.size();

        dir.sub_dir_names.iter().for_each(|subdir| {
            curr_size += self.calculate_dir_size(*subdir);
        });

        curr_size
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
struct Directory {
    parent_dir_name: Option<char>,
    name: char,
    files: Vec<File>,
    sub_dir_names: Vec<char>,
}

impl Directory {
    fn new(parent_dir_name: Option<char>, name: char) -> Self {
        Self {
            name,
            parent_dir_name,
            files: Vec::new(),
            sub_dir_names: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_sub_dir(&mut self, sub_dir_name: char) {
        self.sub_dir_names.push(sub_dir_name);
    }

    fn size(&self) -> i32 {
        let mut size = 0;
        self.files.iter().for_each(|f| {
            size += f.size;
        });

        size
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

impl File {
    fn new(name: String, size: i32) -> Self {
        Self { name, size }
    }
}

impl SeventhPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Seventh Puzzle - No Space Left On Device", "./inputs/7.txt"),
        }
    }

    fn total_size_of_dirs_to_delete(&self, directory_size_limit: i32) -> i32 {
        let line_commands = self.get_line_commands();
        let file_system = FileSystem::build_file_system_from_line_commands(line_commands);

        file_system
            .directories
            .iter()
            .map(|(dir_name, _)| file_system.calculate_dir_size(*dir_name))
            .filter(|dir_size| *dir_size < directory_size_limit)
            .sum()
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
                    let dir_name = re_dir.captures(line).unwrap();
                    line_commads.push(LineCommand::CD(CDKind::Down(
                        *&dir_name[1].chars().next().unwrap(),
                    )));
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
                let dir_name = re_dir.captures(line).unwrap();
                file_system_nodes.push(LSOutput::Directory(*&dir_name[1].chars().next().unwrap()))
            }
        }

        if !file_system_nodes.is_empty() {
            line_commads.push(LineCommand::LS(file_system_nodes))
        }

        line_commads
    }
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInfo;

    use super::*;

    #[test]
    fn total_size_of_dirs_at_most() {
        assert_eq!(
            95437,
            SeventhPuzzle {
                puzzle: get_puzzle_info(),
            }
            .total_size_of_dirs_to_delete(100000)
        )
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
