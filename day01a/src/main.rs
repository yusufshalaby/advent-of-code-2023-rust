fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let mut first_digit: Option<u32> = None;
                let mut last_digit: Option<u32> = None;
                for c in line.chars().filter(|c| c.is_ascii_digit()) {
                    if first_digit.is_none() {
                        first_digit = Some(c.to_digit(10).unwrap());
                    }
                    last_digit = Some(c.to_digit(10).unwrap());
                }
                first_digit.unwrap() * 10 + last_digit.unwrap()
            })
            .sum::<u32>()
    );
}
