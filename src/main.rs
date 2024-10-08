use std::fs::read_to_string;

use aoc2023_1::{numbers_from_line::numbers_from_line, process_line_numbers::process_line_numbers};

fn main() {
    let calibration_document_to_fix =
        read_to_string("input.txt").expect("Could not read from input file");
    let vec_values = calibration_document_to_fix
        .lines()
        .map(|line| {
            let first_number = numbers_from_line(line.chars(), false);
            let last_number = numbers_from_line(line.chars().rev(), true);
            process_line_numbers(first_number, last_number).expect("No numbers in the line")
        })
        .collect::<Vec<u32>>();
    let sum = vec_values.iter().sum::<u32>();
    println!("{sum}");
}