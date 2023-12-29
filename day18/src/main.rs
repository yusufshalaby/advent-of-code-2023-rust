#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let split_line: Vec<&str> = line.splitn(3, " ").collect();
            (
                Direction::new(split_line[0]),
                split_line[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Direction, i32)>>()
}

fn get_coords(input: Vec<(Direction, i32)>) -> Vec<Vec<(Direction, i32)>> {
    let (mut min_row, mut max_row, mut curr_row, mut curr_col) = (0, 0, 0, 0);
    let mut coords: Vec<Vec<(Direction, i32)>> = vec![vec![]];
    for row in input.iter() {
        match row.0 {
            Direction::Up => {
                coords[(curr_row - min_row) as usize].push((Direction::Up, curr_col));
                for i in (curr_row - row.1..curr_row).rev() {
                    if i < min_row {
                        coords.insert(0, vec![(Direction::Up, curr_col)]);
                        min_row = i;
                    } else {
                        coords[(i - min_row) as usize].push((Direction::Up, curr_col));
                    }
                }
                curr_row -= row.1;
            }
            Direction::Down => {
                coords[(curr_row - min_row) as usize].push((Direction::Down, curr_col));
                for i in curr_row + 1..curr_row + row.1 + 1 {
                    if i > max_row {
                        coords.push(vec![(Direction::Down, curr_col)]);
                        max_row = i;
                    } else {
                        coords[(i - min_row) as usize].push((Direction::Down, curr_col));
                    }
                }
                curr_row += row.1;
            }
            Direction::Left => {
                for i in (curr_col - row.1 + 1..curr_col).rev() {
                    coords[(curr_row - min_row) as usize].push((Direction::Left, i));
                }
                curr_col -= row.1;
            }
            Direction::Right => {
                for i in curr_col + 1..curr_col + row.1 {
                    coords[(curr_row - min_row) as usize].push((Direction::Right, i));
                }
                curr_col += row.1;
            }
        }
        // println!("{}, {}", curr_row, curr_col);
    }
    for row in coords.iter_mut() {
        row.sort_by(|a, b| a.1.cmp(&b.1));
    }

    coords
}

fn count_area(coords: Vec<Vec<(Direction, i32)>>) -> i32 {
    let mut area = 0;
    for row in coords.iter() {
        area += 1;
        let mut num_intersections = 1;
        let mut original_direction = row[0].0;
        for i in 1..row.len() {
            assert!(row[i].1 > row[i - 1].1); // should be sorted

            if row[i].1 - row[i - 1].1 == 1 {
                area += 1;
                match (row[i].0, original_direction) {
                    (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => {
                        num_intersections += 1;
                    }
                    _ => (),
                }
                if row[i].0 != row[i - 1].0 {
                    original_direction = row[i-1].0;
                }
            } else {
                if num_intersections % 2 == 0 {
                    area += 1;
                } else {
                    area += row[i].1 - row[i - 1].1;
                }
                num_intersections += 1;
            }
        }
    }
    area
}

fn day18a(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    let coords = get_coords(parsed_input);
    // println!("{:?}", coords);
    count_area(coords)
}

fn day18b(_input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day18a(input));
    println!("{}", day18b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    }

    #[test]
    fn test_18a() {
        let input = input();
        assert_eq!(day18a(input), 62);
    }
}
