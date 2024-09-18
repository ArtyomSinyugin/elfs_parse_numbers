#[cfg(test)]
use crate::{numbers_from_line::numbers_from_line, process_line_numbers::process_line_numbers};

#[test]
fn global_test() {
    let test_data: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    let vec_values = test_data
    .lines()
    .map(|line| {
        let first_number = numbers_from_line(line.chars(), false);
        let last_number = numbers_from_line(line.chars().rev(), true);
        let numbers = process_line_numbers(first_number, last_number).expect("No numbers in the line");
        println!("{numbers}");
        numbers
    })
    .collect::<Vec<u32>>();
    let sum = vec_values.iter().sum::<u32>();
    assert_eq!(sum, 281);
}