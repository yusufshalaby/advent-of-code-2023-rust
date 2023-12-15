fn dayxa(input: &str) -> i32 {
    0
}

fn dayxb(input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", dayxa(input));
    println!("{}", dayxb(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        ""
    }

    #[test]
    fn test_xa() {
        let input = input();
        assert_eq!(dayxa(input), 0);
    }

    #[test]
    fn test_xb() {
        let input = input();
        assert_eq!(dayxb(input), 0);
    }
}
