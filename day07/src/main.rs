use crate::day07a::day07a;
use crate::day07b::day07b;

mod day07a;
mod day07b;


fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day07a(input));
    println!("{}", day07b(input));
}

#[cfg(test)]
mod tests {
    use crate::day07a;
    use crate::day07b;

    fn input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }

    #[test]
    fn test_7a() {
        let input = input();
        assert_eq!(day07a(input), 6440);
    }

    #[test]
    fn test_7b() {
        let input = input();
        assert_eq!(day07b(input), 5905);
    }
}
