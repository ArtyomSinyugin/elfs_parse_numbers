pub fn process_line_numbers(first_number: Option<u32>, last_number: Option<u32>) -> Option<u32> {
    if let Some(first_number) = first_number {
        Some(first_number * 10 + last_number.unwrap())
    } else {
        None
    }
}
