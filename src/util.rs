use std::{fmt::Display, fs};

pub fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Not able to read the file")
}

pub fn print_solution<T: Display>(puzzle_name: &str, first_part_result: T, second_part_result: T) {
    println!("{puzzle_name}: Solution for the first part is: {first_part_result}. Solution for the second part is: {second_part_result}");
}
