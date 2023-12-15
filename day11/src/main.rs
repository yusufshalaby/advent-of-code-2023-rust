fn expand_universe(
    input: &str,
    expansion_size: usize,
) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut numexpandedrows = 0;
    let mut expandedrows = Vec::new();
    let mut foundgalaxycolumn = vec![false; input.lines().next().unwrap().chars().count()];
    let mut originalgalaxypositions = Vec::new();
    for (i, row) in input.lines().enumerate() {
        let mut foundgalaxy = false;
        for (j, col) in row.chars().enumerate() {
            if col == '#' {
                foundgalaxy = true;
                foundgalaxycolumn[j] = true;
                originalgalaxypositions.push((i, j));
            }
        }
        if !foundgalaxy {
            numexpandedrows += expansion_size;
        }
        expandedrows.push(numexpandedrows);
    }
    let mut numexpandedcols = 0;
    let mut expandedcols = Vec::new();
    for foundgalaxy in foundgalaxycolumn.iter() {
        if !foundgalaxy {
            numexpandedcols += expansion_size;
        }
        expandedcols.push(numexpandedcols);
    }
    (originalgalaxypositions, expandedrows, expandedcols)
}

fn sum_of_distances(
    originalgalaxypositions: &Vec<(usize, usize)>,
    expandedrows: &Vec<usize>,
    expandedcols: &Vec<usize>,
) -> i64 {
    let mut newgalaxypositions = Vec::new();
    for (i, j) in originalgalaxypositions {
        newgalaxypositions.push((i + expandedrows[*i], j + expandedcols[*j]));
    }
    let mut result = 0;
    for (i, galaxy) in newgalaxypositions.iter().enumerate() {
        for galaxy2 in newgalaxypositions.iter().skip(i + 1) {
            result += (galaxy.0 as i64 - galaxy2.0 as i64).abs()
                + (galaxy.1 as i64 - galaxy2.1 as i64).abs();
        }
    }
    result
}

fn day11(input: &str, expansion_size: usize) -> i64 {
    let expansion_size = (expansion_size - 1).max(1);
    let (originalgalaxypositions, expandedrows, expandedcols) =
        expand_universe(input, expansion_size);
    sum_of_distances(&originalgalaxypositions, &expandedrows, &expandedcols) as i64
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day11(input, 1));
    println!("{}", day11(input, 1_000_000));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    #[test]
    fn test_11a() {
        let input = input();
        assert_eq!(day11(input, 1), 374);
    }

    #[test]
    fn test_11b_10() {
        let input = input();
        assert_eq!(day11(input, 10), 1030);
    }

    #[test]
    fn test_11b_100() {
        let input = input();
        assert_eq!(day11(input, 100), 8410);
    }
}
