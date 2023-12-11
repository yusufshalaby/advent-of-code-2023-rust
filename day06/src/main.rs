fn parse_input_5a(input: &str) -> Vec<(i32, i32)> {
    let lines: Vec<&str> = input.lines().collect();
    lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .zip(
            lines[1]
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<i32>().unwrap()),
        )
        .collect::<Vec<_>>()
}

fn day6a(input: &str) -> i32 {
    let mut result = 1;
    for (time, distance) in parse_input_5a(input) {
        let mut val = 0;
        for button_duration in 1..time {
            if (time - button_duration) * button_duration > distance {
                val += 1;
            }
        }
        result *= val;
    }
    result
}

fn parse_input_5b(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .fold("".to_string(), |a, b| a + b)
        .parse::<i64>()
        .unwrap();
    let dist = lines[1]
        .split_whitespace()
        .skip(1)
        .fold("".to_string(), |a, b| a + b)
        .parse::<i64>()
        .unwrap();
    (time, dist)
}

fn day6b(input: &str) -> i64 {
    let (time, distance) = parse_input_5b(input);
    let mut val = 0;
    for button_duration in 1..time {
        if (time - button_duration) * button_duration > distance {
            val += 1;
        }
    }
    val
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day6a(input));
    println!("{}", day6b(input));
}

#[cfg(test)]
mod tests {
    use crate::day6a;
    use crate::day6b;

    fn input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }

    #[test]
    fn test_6a() {
        let input = input();
        assert_eq!(day6a(input), 288);
    }

    #[test]
    fn test_6b() {
        let input = input();
        assert_eq!(day6b(input), 71503);
    }
}
