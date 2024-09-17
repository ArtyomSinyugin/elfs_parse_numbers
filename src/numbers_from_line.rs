pub fn numbers_from_line(line: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    let mut temp_word = String::new();
    let ref_numbers: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let mut iter_line = line.chars();
    while let Some(char) = iter_line.next() {
        if let Some(digit) = char.to_digit(10) {
            temp_word.clear();
            vec.push(digit);
        } else {
            temp_word.push(char);
            for i in 0..ref_numbers.len() {
                if temp_word.contains(ref_numbers[i]) {
                    vec.push(i as u32);
                    match ref_numbers[i] {
                        "one" => temp_word = temp_word.split_once("on").unwrap().1.to_string(),
                        "two" =>  temp_word = temp_word.split_once("tw").unwrap().1.to_string(), 
                        "three" => temp_word = temp_word.split_once("thre").unwrap().1.to_string(), 
                        "nine" => temp_word = temp_word.split_once("nin").unwrap().1.to_string(), 
                        _ => temp_word.clear(), 
                    }
                }
            }
        }
    }
    vec
}