use core::iter::Zip;
use std::iter::zip;

fn split_and_zip(
    input: &Vec<String>,
    index: usize,
) -> Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>> {
    let (leftside, rightside) = input.split_at(index);
    let mut leftside = leftside.to_vec();
    let mut rightside = rightside.to_vec();
    leftside.reverse();
    if leftside.len() > rightside.len() {
        leftside = leftside[..rightside.len()].to_vec();
    } else if rightside.len() > leftside.len() {
        rightside = rightside[..leftside.len()].to_vec();
    }
    zip(leftside, rightside)
}

// my pre 13b solution to 13a
// fn valid_index(input: &Vec<String>, index: usize) -> bool {
//     split_and_zip(input, index).all(|(left, right)| left == right)
// }

fn valid_index(input: &Vec<String>, index: usize, differences: usize) -> bool {
    split_and_zip(input, index)
        .map(|(left, right)| {
            zip(left.chars(), right.chars())
                .fold(0, |acc, (l, r)| if l != r { acc + 1 } else { acc })
        })
        .sum::<usize>()
        == differences
}

fn find_mirror(input: Vec<String>, differences: usize) -> usize {
    if let Some(result) = (1..input.len()).find(|i| valid_index(&input, *i, differences)) {
        return result;
    }
    0
}

fn day13(input: &str, differences: usize) -> usize {
    let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let cols = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    return 100 * find_mirror(rows, differences) + find_mirror(cols, differences);
}

fn day13total(input: &str, differences: usize) -> usize {
    input
        .split("\n\n")
        .fold(0, |acc, group| acc + day13(group, differences))
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day13total(input, 0));
    println!("{}", day13total(input, 1));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input1() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
    }

    fn input2() -> &'static str {
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    #[test]
    fn test_13a1() {
        let input = input1();
        assert_eq!(day13(input, 0), 5);
    }

    #[test]
    fn test_13a2() {
        let input = input2();
        assert_eq!(day13(input, 0), 400);
    }

    #[test]
    fn test_13atotal() {
        let input1 = input1();
        let input2 = input2();
        let input = &format!("{}\n\n{}", input1, input2);
        assert_eq!(day13total(input, 0), 405);
    }
}
