use std::fs::read_to_string;

fn main() {
    let calibration_document = read_to_string("input.txt")
        .expect("Could not read from input file");
    let vec_values = calibration_document
        .lines()
        .map(|line| {
            let numbers = numbers_from_line(line);
            process_line_numbers(numbers)
    }).collect::<Vec<u32>>();
    let sum = vec_values.iter().sum::<u32>();
    println!("{sum}");
}

fn numbers_from_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .filter_map(|char|{
            char.to_digit(10)
        }).collect::<Vec<u32>>()    
}

fn process_line_numbers(numbers: Vec<u32>) -> u32 {
    if numbers.len() > 1 {
        let first = numbers.first().expect("Could not find first digit");
        let last = numbers.last().expect("Could not find last digit");
        format!("{}{}", first, last).parse::<u32>().expect("Could not parse number into two digits")
    } else {
        format!("{}{}", numbers[0], numbers[0]).parse::<u32>().expect("Could not parse number into two digits")
    }
}