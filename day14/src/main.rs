use std::collections::HashMap;

//implement Copy trait
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
enum Rock {
    Round,
    Cube,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Platform {
    rocks: Vec<Vec<Option<Rock>>>,
    direction: Direction,
}

impl Platform {
    fn new(input: &str) -> Platform {
        Platform {
            rocks: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'O' => Some(Rock::Round),
                            '#' => Some(Rock::Cube),
                            '.' => None,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            direction: Direction::West,
        }
    }

    fn transpose(&mut self) {
        // transpose the rocks
        self.rocks = (0..self.rocks[0].len())
            .map(|i| {
                self.rocks
                    .iter()
                    .map(|row| *row.iter().nth(i).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }

    fn reverse_columns(&mut self) {
        // reverse the rocks
        self.rocks = self
            .rocks
            .iter()
            .map(|row| row.iter().rev().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    fn reverse_rows(&mut self) {
        // reverse the rocks
        self.rocks.reverse();
    }

    fn change_direction(&mut self, direction: Direction) {
        if self.direction == direction {
            return;
        }
        self.direction = match self.direction {
            Direction::North => match direction {
                Direction::West => {
                    self.transpose();
                    Direction::West
                }
                _ => unreachable!(),
            },
            Direction::East => match direction {
                Direction::North => {
                    self.transpose();
                    self.reverse_rows();
                    Direction::North
                }
                Direction::West => {
                    self.reverse_columns();
                    Direction::West
                }
                _ => unreachable!(),
            },
            Direction::South => match direction {
                Direction::East => {
                    self.transpose();
                    self.reverse_rows();
                    self.reverse_columns();
                    Direction::East
                }
                _ => unreachable!(),
            },
            Direction::West => match direction {
                Direction::North => {
                    self.transpose();
                    Direction::North
                }
                Direction::South => {
                    self.reverse_rows();
                    self.transpose();
                    Direction::South
                }
                _ => unreachable!(),
            },
        }
    }

    fn tilt(&mut self) {
        for i in 0..self.rocks.len() {
            let mut last_empty_space = 0;
            for j in 0..self.rocks[i].len() {
                if let Some(rock) = self.rocks[i][j] {
                    match rock {
                        Rock::Round => {
                            self.rocks[i][j] = None;
                            self.rocks[i][last_empty_space] = Some(Rock::Round);
                            last_empty_space += 1;
                        }
                        Rock::Cube => {
                            last_empty_space = j + 1;
                        }
                    }
                }
            }
        }
    }

    fn get_load(mut self) -> i32 {
        self.change_direction(Direction::West);
        let mut final_rocks = self.rocks.clone();
        final_rocks.reverse();
        final_rocks.iter().enumerate().fold(0, |acc, (i, row)| {
            row.iter()
                .filter(|rock| {
                    if let Some(rock) = rock {
                        if let Rock::Round = rock {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
                .count() as i32
                * (i as i32 + 1)
                + acc
        })
    }

    fn get_str(&self) -> String {
        self.rocks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|rock| match rock {
                        Some(Rock::Round) => 'O',
                        Some(Rock::Cube) => '#',
                        None => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn day14a(input: &str) -> i32 {
    let mut platform = Platform::new(input);
    platform.change_direction(Direction::North);
    platform.tilt();
    platform.get_load()
}

fn day14b(input: &str) -> i32 {
    let mut platform = Platform::new(input);
    let mut states = HashMap::new();
    let mut early_exit: Option<usize> = None;
    for i in 0..1_000_000_000 {
        if early_exit.map(|value| value == i).unwrap_or(false) {
            break;
        }
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            platform.change_direction(direction);
            platform.tilt();
        }
        let platform_state = platform.get_str();
        if let Some(first) = states.get(&platform_state) {
            let cycle = i - first;
            early_exit.replace(i + (1_000_000_000 - i) % cycle);
        }
        states.insert(platform_state, i);
    }
    platform.get_load()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day14a(input));
    println!("{}", day14b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    }

    #[test]
    fn test_xa() {
        let input = input();
        assert_eq!(day14a(input), 136);
    }

    #[test]
    fn test_xb() {
        let input = input();
        assert_eq!(day14b(input), 64);
    }
}
