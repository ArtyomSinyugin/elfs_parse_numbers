const NUMBERS: [&str; 10] = ["0", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const REV_NUMBERS: [&str; 10] = ["0", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

pub fn numbers_from_line<T: Iterator<Item = char>>(mut line: T, reverse: bool) -> Option<u32> {
    let mut temp_word = String::new();
    let ref_numbers = if reverse { REV_NUMBERS } else { NUMBERS };
    while let Some(char) = line.next() {
        if let Some(digit) = char.to_digit(10) {
            return Some(digit);
        } else {
            temp_word.push(char);
            for i in 0..ref_numbers.len() {
                if temp_word.contains(ref_numbers[i]) {
                    return Some(i as u32);
                }
            }
        }
    }
    None
}