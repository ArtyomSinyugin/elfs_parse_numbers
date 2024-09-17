#[test]
fn test_data() {
    use crate::{numbers_from_line::numbers_from_line, process_line_numbers::process_line_numbers};

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
        let numbers = numbers_from_line(line);
        let numbers = process_line_numbers(numbers);
        println!("{numbers}");
        numbers
    })
    .collect::<Vec<u32>>();
    let sum = vec_values.iter().sum::<u32>();
    assert_eq!(sum, 281);
}