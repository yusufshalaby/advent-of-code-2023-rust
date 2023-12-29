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
            let split_line: Vec<&str> = line.splitn(3, ' ').collect();
            (
                Direction::new(split_line[0]),
                split_line[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Direction, i32)>>()
}

fn get_coords(input: &[(Direction, i32)]) -> Vec<(i32, i32)> {
    let (mut curr_row, mut curr_col) = (0, 0);
    let mut coords: Vec<(i32, i32)> = vec![(curr_row, curr_col)];
    for row in input {
        match row.0 {
            Direction::Up => {
                curr_row -= row.1 + 1;
            }
            Direction::Down => {
                curr_row += row.1 + 1;
            }
            Direction::Left => {
                curr_col -= row.1 + 1;
            }
            Direction::Right => {
                curr_col += row.1 + 1;
            }
        }
        coords.push((curr_row, curr_col));
        println!("{curr_row}, {curr_col}");
    }

    assert_eq!(coords[0], coords[coords.len() - 1]);
    coords
}

fn polynomial_area(coords: &[(i32, i32)]) -> i32 {
    let mut area = 0;
    for i in 0..coords.len() - 1 {
        area += coords[i].0 * coords[i + 1].1 - coords[i + 1].0 * coords[i].1;
        println!(
            "{:?} {:?} => {:?} - {:?} = {:?} => {:?}",
            coords[i],
            coords[i + 1],
            coords[i].0 * coords[i + 1].1,
            coords[i].1 * coords[i + 1].0,
            coords[i].0 * coords[i + 1].1 - coords[i + 1].0 * coords[i].1,
            area
        );
    }

    area.abs() / 2
}

fn day18a(input: &str) -> i32 {
    let parsed_input = parse_input(input);
    let coords = get_coords(&parsed_input);
    println!("{coords:?}");
    polynomial_area(&coords)
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
