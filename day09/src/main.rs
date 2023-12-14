use itertools::izip;

fn find_rightmost_value(input: Vec<i32>) -> i32 {
    let mut row = input.clone();
    let mut pyramid = Vec::new();
    while !row.iter().all(|&x| x == 0) {
        let newrow: Vec<_> = izip!(&row[..row.len() - 1], &row[1..])
            .map(|(a, b)| b - a)
            .collect::<_>();
        row = newrow.clone();
        pyramid.push(row.clone());
    }
    input.last().unwrap() + pyramid.iter().map(|row| row.last().unwrap()).sum::<i32>()
}

fn find_leftmost_value(input: Vec<i32>) -> i32 {
    let mut row = input.clone();
    let mut pyramid = Vec::new();
    while !row.iter().all(|&x| x == 0) {
        let newrow: Vec<_> = izip!(&row[..row.len() - 1], &row[1..])
            .map(|(a, b)| b - a)
            .collect::<_>();
        row = newrow.clone();
        pyramid.push(row.clone());
    }

    let mut prev_leftmost = 0;
    for row in pyramid.iter().rev().skip(1) {
        prev_leftmost = row[0] - prev_leftmost;
    }
    input.first().unwrap() - prev_leftmost
}

fn day9a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let parsed_line = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            find_rightmost_value(parsed_line)
        })
        .sum()
}

fn day9b(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let parsed_line = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            find_leftmost_value(parsed_line)
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day9a(input));
    println!("{}", day9b(input));
}

#[cfg(test)]
mod tests {
    use crate::day9a;
    use crate::day9b;

    fn input() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }

    #[test]
    fn test_9a() {
        let input = input();
        assert_eq!(day9a(input), 114);
    }
    #[test]
    fn test_9b() {
        let input = input();
        assert_eq!(day9b(input), 2);
    }
}
