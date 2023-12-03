use std::collections::HashMap;

fn main() {
    let word2digit: HashMap<char, Vec<(&str, u32)>> = HashMap::from([
        ('1', Vec::from([("", 1)])),
        ('2', Vec::from([("", 2)])),
        ('3', Vec::from([("", 3)])),
        ('4', Vec::from([("", 4)])),
        ('5', Vec::from([("", 5)])),
        ('6', Vec::from([("", 6)])),
        ('7', Vec::from([("", 7)])),
        ('8', Vec::from([("", 8)])),
        ('9', Vec::from([("", 9)])),
        ('o', Vec::from([("ne", 1)])),
        ('t', Vec::from([("wo", 2), ("hree", 3)])),
        ('f', Vec::from([("our", 4), ("ive", 5)])),
        ('s', Vec::from([("ix", 6), ("even", 7)])),
        ('e', Vec::from([("ight", 8)])),
        ('n', Vec::from([("ine", 9)])),
    ]);

    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let mut first_digit: Option<u32> = None;
                let mut last_digit: Option<u32> = None;
                for (i, c) in line.chars().enumerate() {
                    if let Some(options) = word2digit.get(&c) {
                        for (word, digit) in options {
                            if *word == line.get(i + 1..i + 1 + word.len()).unwrap_or("") {
                                if first_digit.is_none() {
                                    first_digit = Some(*digit);
                                }
                                last_digit = Some(*digit);
                                break;
                            }
                        }
                    }
                }
                return first_digit.unwrap() * 10 + last_digit.unwrap();
            })
            .sum::<u32>()
    );
}
