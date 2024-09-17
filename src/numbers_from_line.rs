pub fn numbers_from_line(line: &str) -> Vec<(u32, usize)> {
    let mut output: Vec<(u32, usize)> = Vec::new();

    let ref_numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9", "0",
    ];

    for ref_number in ref_numbers {
        let order_number = line.find(ref_number);
        let r_order_number = line.rfind(ref_number);
        let output_u32 = str_to_digit(ref_number);

        if let Some(order_number) = order_number {
            output.push((output_u32, order_number));
        }

        if r_order_number != order_number {
            if let Some(r_order_number) = r_order_number {
                output.push((output_u32, r_order_number));
            }
        }
    }

    if output.len() > 1 {
        output.sort_by(|a, b| a.1.cmp(&b.1));
    }

    output
}

fn str_to_digit(str: &str) -> u32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x
            .parse()
            .expect("Could not find number. Check vector of numbers in fn numbers_from_line"),
    }
}
