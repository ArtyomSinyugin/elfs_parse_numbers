pub fn process_line_numbers(numbers: Vec<u32>) -> u32 {
    if numbers.len() > 1 {
        let first = numbers.first().expect("Could not find first digit") * 10;
        let last = numbers.last().expect("Could not find last digit");
        return first + last;
    } else {
        return numbers[0] * 10 + numbers[0];
    }
}
