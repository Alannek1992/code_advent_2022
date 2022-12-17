use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use itertools::Itertools;
use regex::Regex;

use crate::{util::print_solution, PuzzleInfo, Solution};

pub struct SeventhPuzzle {
    puzzle: PuzzleInfo,
}

impl Solution for SeventhPuzzle {
    fn solution(&self) {
        print_solution(
            &self.puzzle.name,
            self.total_size_of_dirs_to_delete(100000),
            self.size_of_dir_to_delete(70000000, 30000000),
        );
    }
}

#[derive(Debug)]
struct FileSystem {
    directories: HashMap<u64, Directory>,
}

impl FileSystem {
    fn build_file_system_from_line_commands(line_commands: Vec<LineCommand>) -> Self {
        let mut directories = HashMap::new();
        let root_dir = Directory::new("/", "/");
        directories.insert(Self::get_hash(&root_dir.path), root_dir);
        let mut current_path = String::from("/");

        for line_command in line_commands {
            match line_command {
                LineCommand::CD(kind) => match kind {
                    CDKind::Up => {
                        let current_dir = directories.get(&Self::get_hash(&current_path)).unwrap();
                        current_path = String::from(
                            &current_path[..(current_path.len() - current_dir.name.len() - 1)],
                        );
                    }
                    CDKind::Down(dir_name) => {
                        current_path.push_str(&format!("{}/", dir_name));
                    }
                },
                LineCommand::LS(output) => {
                    for ls_out in output {
                        match ls_out {
                            LSOutput::File(size) => {
                                directories
                                    .get_mut(&FileSystem::get_hash(&current_path))
                                    .unwrap()
                                    .add_file(File::new(size));
                            }
                            LSOutput::Directory(dir_name) => {
                                let new_dir_path = format!("{}{}/", current_path, dir_name);
                                let new_dir_path_hash = Self::get_hash(&new_dir_path);

                                if !directories.contains_key(&new_dir_path_hash) {
                                    let current_dir_hash = Self::get_hash(&current_path);
                                    let current_dir_ref =
                                        directories.get_mut(&current_dir_hash).unwrap();

                                    current_dir_ref.add_sub_dir(&new_dir_path);
                                    directories.insert(
                                        new_dir_path_hash,
                                        Directory::new(&new_dir_path, &dir_name),
                                    );
                                }
                            }
                        };
                    }
                }
            }
        }

        FileSystem { directories }
    }

    fn calculate_dir_size(&self, dir_hash: u64) -> i32 {
        let dir = self.directories.get(&dir_hash).expect("Dir does not exist");
        let mut curr_size = dir.size();

        dir.sub_dir_paths.iter().for_each(|subdir| {
            curr_size += self.calculate_dir_size(Self::get_hash(&subdir));
        });

        curr_size
    }

    fn get_hash(dir_name: &str) -> u64 {
        let mut h = DefaultHasher::new();
        dir_name.hash(&mut h);
        h.finish()
    }
}

#[derive(Debug)]
struct Directory {
    path: String,
    name: String,
    files: Vec<File>,
    sub_dir_paths: Vec<String>,
}

impl Directory {
    fn new(path: &str, name: &str) -> Self {
        Self {
            path: String::from(path),
            name: String::from(name),
            files: Vec::new(),
            sub_dir_paths: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_sub_dir(&mut self, sub_dir_path: &str) {
        self.sub_dir_paths.push(String::from(sub_dir_path));
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
    size: i32,
}

impl File {
    fn new(size: i32) -> Self {
        Self { size }
    }
}

#[derive(Debug)]
enum LineCommand {
    CD(CDKind),
    LS(Vec<LSOutput>),
}

#[derive(Debug)]
enum CDKind {
    Up,
    Down(String),
}

#[derive(Debug)]
enum LSOutput {
    File(i32),
    Directory(String),
}

impl SeventhPuzzle {
    pub fn new() -> Self {
        Self {
            puzzle: PuzzleInfo::new("Seventh Puzzle - No Space Left On Device", "./inputs/7.txt"),
        }
    }

    fn total_size_of_dirs_to_delete(&self, directory_size_limit: i32) -> i32 {
        self.get_dir_sizes()
            .iter()
            .filter(|dir_size| **dir_size < directory_size_limit)
            .sum()
    }

    fn size_of_dir_to_delete(&self, total_space: i32, space_needed: i32) -> i32 {
        let dir_sizes = self.get_dir_sizes();
        let largest_dir = dir_sizes.iter().max().unwrap();
        let available_space = total_space - largest_dir;

        *dir_sizes
            .iter()
            .sorted()
            .find(|size| (**size + available_space) > space_needed)
            .unwrap()
    }

    fn get_dir_sizes(&self) -> Vec<i32> {
        let line_commands = self.get_line_commands();
        let file_system = FileSystem::build_file_system_from_line_commands(line_commands);

        file_system
            .directories
            .iter()
            .map(|(dir_name, _)| file_system.calculate_dir_size(*dir_name))
            .collect()
    }

    fn get_line_commands(&self) -> Vec<LineCommand> {
        let re_command = Regex::new(r"\$.*").unwrap();
        let re_cd_down = Regex::new(r".*cd [a-z]").unwrap();
        let re_cd_up = Regex::new(r".*cd ..").unwrap();
        let re_file = Regex::new(r"(\d+) (.*)").unwrap();
        let re_dir = Regex::new(r".* (\w+)").unwrap();

        let mut line_commads = Vec::new();
        let mut file_system_nodes = Vec::new();

        for line in self.puzzle.input.lines() {
            let line = line.trim();

            if re_command.is_match(line) {
                if !file_system_nodes.is_empty() {
                    line_commads.push(LineCommand::LS(file_system_nodes.drain(..).collect()))
                }
                if re_cd_down.is_match(line) {
                    let dir_name = re_dir.captures(line).unwrap();
                    line_commads.push(LineCommand::CD(CDKind::Down(String::from(&dir_name[1]))));
                } else if re_cd_up.is_match(line) {
                    line_commads.push(LineCommand::CD(CDKind::Up));
                }

                continue;
            }

            if let Some(file) = re_file.captures(line) {
                file_system_nodes.push(LSOutput::File(*&file[1].parse().unwrap()))
            } else {
                let dir_name = re_dir.captures(line).unwrap();
                file_system_nodes.push(LSOutput::Directory(String::from(&dir_name[1])));
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
    fn size_of_dir_to_delete() {
        assert_eq!(
            24933642,
            SeventhPuzzle {
                puzzle: get_puzzle_info(),
            }
            .size_of_dir_to_delete(70000000, 30000000)
        )
    }

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
