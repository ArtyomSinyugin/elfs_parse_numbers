pub fn process_line_numbers(numbers: Vec<(u32, usize)>) -> u32 {
    if numbers.len() > 1 {
        let first = numbers.first().expect("Could not find first digit");
        let last = numbers.last().expect("Could not find last digit");
        format!("{}{}", first.0, last.0)
            .parse::<u32>()
            .expect("Could not parse number into two digits")
    } else {
        format!("{}{}", numbers[0].0, numbers[0].0)
            .parse::<u32>()
            .expect("Could not parse number into two digits")
    }
}
