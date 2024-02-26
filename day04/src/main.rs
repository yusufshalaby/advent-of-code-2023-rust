use std::collections::HashMap;

fn day4a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, my_numbers) =
                line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let count = winning_numbers
                .trim()
                .split(' ')
                .filter(|&n| {
                    !n.is_empty()
                        && my_numbers
                            .trim()
                            .split(' ')
                            .collect::<Vec<_>>()
                            .contains(&n)
                })
                .count();
            if count == 0 {
                return 0;
            }
            2i32.pow((count - 1) as u32)
        })
        .sum()
}

fn day4b(input: &str) -> i32 {
    let mut num_cards: HashMap<usize, u32> = HashMap::new();
    input.lines().enumerate().map(|(i, line)| {
        let (winning_numbers, my_numbers) =
            line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let count = winning_numbers
            .trim()
            .split(' ')
            .filter(|&n| {
                !n.is_empty()
                    && my_numbers
                        .trim()
                        .split(' ')
                        .collect::<Vec<_>>()
                        .contains(&n)
            })
            .count();
        let multiplier = *num_cards.entry(i).or_insert(1);
        for j in i+1..i+1+count{
            *num_cards.entry(j).or_insert(1) += multiplier;
        }
        multiplier as i32
    }).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day4a(input));
    println!("{}", day4b(input));
}

#[cfg(test)]
mod tests {
    use crate::day4a;
    use crate::day4b;

    fn input() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    }

    #[test]
    fn test_4a() {
        let input = input();
        assert_eq!(day4a(input), 13);
    }

    #[test]
    fn test_4b() {
        let input = input();
        assert_eq!(day4b(input), 30);
    }
}
