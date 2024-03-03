use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle {
    pattern: String,
    groupsizes: Vec<usize>,
}

impl Puzzle {
    fn new_a(input: &str) -> Self {
        let (pattern, groupsizes) = input.split_once(' ').unwrap();
        Self {
            pattern: pattern.chars().collect(),
            groupsizes: groupsizes
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn new_b(input: &str) -> Self {
        let (pattern, groupsizes) = input.split_once(' ').unwrap();
        let pattern = std::iter::repeat(pattern)
            .take(5)
            .collect::<Vec<_>>()
            .join("?");
        let groupsizes = std::iter::repeat(
            groupsizes
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        )
        .take(5)
        .flatten()
        .collect::<Vec<_>>();
        Self {
            pattern,
            groupsizes,
        }
    }

    fn solve(
        &self,
        si: usize,
        ri: usize,
        neighbor: bool,
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if let Some(&v) = cache.get(&(si, ri)) {
            return v;
        }
        // out of groups and pattern is exhausted or only remaining are . or ?
        if ri == self.groupsizes.len()
            && (si >= self.pattern.len()
                || self.pattern[si..].chars().all(|c| c == '.' || c == '?'))
        {
            cache.insert((si, ri), 1);
            return 1;
        }

        // overshot pattern or groups
        if si >= self.pattern.len() || ri >= self.groupsizes.len() {
            cache.insert((si, ri), 0);
            return 0;
        }

        let val;
        match self.pattern[si..].chars().next() {
            Some('.') => return self.solve(si + 1, ri, false, cache),
            Some('#') => {
                if neighbor {
                    return 0;
                } else if (si + self.groupsizes[ri] <= self.pattern.len())
                    && self.pattern[si..si + self.groupsizes[ri]]
                        .chars()
                        .all(|c| c == '#' || c == '?')
                {
                    val = self.solve(si + self.groupsizes[ri], ri + 1, true, cache);
                } else {
                    val = 0;
                }
            }
            Some('?') => {
                if neighbor {
                    return self.solve(si + 1, ri, false, cache);
                } else if (si + self.groupsizes[ri] <= self.pattern.len())
                    && self.pattern[si..si + self.groupsizes[ri]]
                        .chars()
                        .all(|c| c == '#' || c == '?')
                {
                    val = self.solve(si + 1, ri, false, cache)
                        + self.solve(si + self.groupsizes[ri], ri + 1, true, cache);
                } else {
                    val = self.solve(si + 1, ri, false, cache);
                }
            }
            None => {
                val = 1;
            }
            _ => unreachable!(),
        }
        cache.insert((si, ri), val);
        val
    }
}

fn day12a(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let puzzle = Puzzle::new_a(line);
            let mut cache = HashMap::new();
            puzzle.solve(0, 0, false, &mut cache) as i32
        })
        .sum()
}

fn day12b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let puzzle = Puzzle::new_b(line);
            let mut cache = HashMap::new();
            puzzle.solve(0, 0, false, &mut cache) as i64
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day12a(input));
    println!("{}", day12b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    }

    #[test]
    fn test_12a() {
        let input = input();
        assert_eq!(day12a(input), 21);
    }

    #[test]
    fn test_12b() {
        let input = input();
        assert_eq!(day12b(input), 525152);
    }
}
