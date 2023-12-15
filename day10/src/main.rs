#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
enum Pipe {
    NorthSouth(bool),
    EastWest(bool),
    NorthEast(bool),
    NorthWest(bool),
    SouthEast(bool),
    SouthWest(bool),
    Start(bool),
}

impl Pipe {
    fn find_next_direction(&self, prev_direction: &Direction) -> Option<Direction> {
        match self {
            Pipe::NorthSouth(_) => match prev_direction {
                Direction::North => Some(Direction::North),
                Direction::South => Some(Direction::South),
                _ => None,
            },
            Pipe::EastWest(_) => match prev_direction {
                Direction::East => Some(Direction::East),
                Direction::West => Some(Direction::West),
                _ => None,
            },
            Pipe::NorthEast(_) => match prev_direction {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Pipe::NorthWest(_) => match prev_direction {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Pipe::SouthEast(_) => match prev_direction {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Pipe::SouthWest(_) => match prev_direction {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            Pipe::Start(_) => match prev_direction {
                Direction::North => Some(Direction::North),
                Direction::East => Some(Direction::East),
                Direction::South => Some(Direction::South),
                Direction::West => Some(Direction::West),
            },
        }
    }
}

fn find_next_index(
    map: &Vec<Vec<Option<Pipe>>>,
    direction: &Direction,
    current_index: (usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Direction::North => {
            if current_index.0 == 0 {
                None
            } else {
                Some((current_index.0 - 1, current_index.1))
            }
        }
        Direction::East => {
            if current_index.1 == map[0].len() - 1 {
                None
            } else {
                Some((current_index.0, current_index.1 + 1))
            }
        }
        Direction::South => {
            if current_index.0 == map.len() - 1 {
                None
            } else {
                Some((current_index.0 + 1, current_index.1))
            }
        }
        Direction::West => {
            if current_index.1 == 0 {
                None
            } else {
                Some((current_index.0, current_index.1 - 1))
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Option<Pipe>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '-' => Some(Pipe::EastWest(false)),
                    '|' => Some(Pipe::NorthSouth(false)),
                    'L' => Some(Pipe::NorthEast(false)),
                    'J' => Some(Pipe::NorthWest(false)),
                    'F' => Some(Pipe::SouthEast(false)),
                    '7' => Some(Pipe::SouthWest(false)),
                    'S' => Some(Pipe::Start(true)),
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

fn find_start(map: &mut Vec<Vec<Option<Pipe>>>) -> (Option<(usize, usize)>, Option<Direction>) {
    let mut startindex = None;
    let mut startdirection = None;
    for (x, row) in map.iter().enumerate() {
        for (y, pipe) in row.iter().enumerate() {
            if let Some(Pipe::Start(_)) = pipe {
                startindex = Some((x, y));
            }
        }
    }
    if let Some(startindex) = startindex {
        let mut valid_directions = vec![];
        for direction in vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            if let Some(next_index) = find_next_index(map, &direction, startindex) {
                if let Some(pipe) = map.get(next_index.0).unwrap().get(next_index.1).unwrap() {
                    if let Some(_) = pipe.find_next_direction(&direction) {
                                valid_directions.push(direction);
                    }
                }
            }
        }
        let direction1 = &valid_directions[0];
        let direction2 = &valid_directions[1];
        match (direction1, direction2) {
            (Direction::North, Direction::East) => {
                map[startindex.0][startindex.1] = Some(Pipe::NorthEast(true));
                startdirection = Some(Direction::North);
            }
            (Direction::North, Direction::West) => {
                map[startindex.0][startindex.1] = Some(Pipe::NorthWest(true));
                startdirection = Some(Direction::North);
            }
            (Direction::South, Direction::East) => {
                map[startindex.0][startindex.1] = Some(Pipe::SouthEast(true));
                startdirection = Some(Direction::South);
            }
            (Direction::South, Direction::West) => {
                map[startindex.0][startindex.1] = Some(Pipe::SouthWest(true));
                startdirection = Some(Direction::South);
            }
            (Direction::North, Direction::South) => {
                map[startindex.0][startindex.1] = Some(Pipe::NorthSouth(true));
                startdirection = Some(Direction::North);
            }
            (Direction::East, Direction::West) => {
                map[startindex.0][startindex.1] = Some(Pipe::EastWest(true));
                startdirection = Some(Direction::East);
            }
            _ => panic!("Invalid start"),
        }
    }
    (startindex, startdirection)
}

fn travel_pipe(
    map: &mut Vec<Vec<Option<Pipe>>>,
    direction: &Direction,
    current_index: (usize, usize),
    current_distance: i32,
) -> ((usize, usize), i32) {
    let pipe = map
        .get_mut(current_index.0)
        .unwrap()
        .get_mut(current_index.1)
        .unwrap();
    if let Some(pipe) = pipe {
        match pipe {
            Pipe::NorthSouth(false) => *pipe = Pipe::NorthSouth(true),
            Pipe::EastWest(false) => *pipe = Pipe::EastWest(true),
            Pipe::NorthEast(false) => *pipe = Pipe::NorthEast(true),
            Pipe::NorthWest(false) => *pipe = Pipe::NorthWest(true),
            Pipe::SouthEast(false) => *pipe = Pipe::SouthEast(true),
            Pipe::SouthWest(false) => *pipe = Pipe::SouthWest(true),
            Pipe::Start(false) => *pipe = Pipe::Start(true),
            Pipe::NorthSouth(true) => return (current_index, current_distance),
            Pipe::EastWest(true) => return (current_index, current_distance),
            Pipe::NorthEast(true) => return (current_index, current_distance),
            Pipe::NorthWest(true) => return (current_index, current_distance),
            Pipe::SouthEast(true) => return (current_index, current_distance),
            Pipe::SouthWest(true) => return (current_index, current_distance),
            Pipe::Start(true) => return (current_index, current_distance),
        };
        if let Some(next_direction) = pipe.find_next_direction(direction) {
            if let Some(next_index) = find_next_index(map, &next_direction, current_index) {
                travel_pipe(map, &next_direction, next_index, current_distance + 1)
            } else {
                // dead end because reached edge of map
                (current_index, current_distance)
            }
        } else {
            // dead end because reached non compatible pipe
            (current_index, current_distance)
        }
    } else {
        // dead end because reached empty space
        (current_index, current_distance)
    }
}

fn traverse_map(map: &mut Vec<Vec<Option<Pipe>>>) -> (i32, Vec<Vec<Option<Pipe>>>) {
    let (start_index, start_direction) = find_start(map);

    if let Some(next_direction) = Pipe::Start(true).find_next_direction(&start_direction.unwrap()) {
        let next_index = find_next_index(&map, &next_direction, start_index.unwrap());
        let mut map = map.clone();
        if let Some(next_index) = next_index {
            let (loop_index, loop_distance) = travel_pipe(&mut map, &next_direction, next_index, 1);
            if loop_index == start_index.unwrap() {
                return (loop_distance / 2, map);
            }
        }
    }
    panic!("No loop found");
}

fn day10a(input: &str) -> i32 {
    let mut map = parse_input(input);
    let (distance, _) = traverse_map(&mut map);
    distance
}

fn day10b(input: &str) -> i32 {
    let mut map = parse_input(input);
    let (_, map) = traverse_map(&mut map);
    let mut result = 0;
    for row in map.iter() {
        let mut curr_row_counter = 0;
        let mut connected_pipe: Option<Pipe> = None;
        for pipe in row.iter() {
            if let Some(pipe) = pipe {
                match pipe {
                    Pipe::NorthSouth(true) => {
                        curr_row_counter += 1;
                        connected_pipe = None;
                    }
                    Pipe::SouthEast(true) => {
                        curr_row_counter += 1;
                        connected_pipe = Some(Pipe::SouthEast(true));
                    }
                    Pipe::SouthWest(true) => match connected_pipe {
                        Some(Pipe::NorthEast(true)) => (),
                        _ => curr_row_counter += 1,
                    },
                    Pipe::Start(true) => {
                        curr_row_counter += 1;
                        connected_pipe = None;
                    }
                    Pipe::EastWest(true) => (),
                    Pipe::NorthEast(true) => {
                        curr_row_counter += 1;
                        connected_pipe = Some(Pipe::NorthEast(true));
                    }
                    Pipe::NorthWest(true) => match connected_pipe {
                        Some(Pipe::SouthEast(true)) => (),
                        _ => curr_row_counter += 1,
                    },
                    _ => {
                        if curr_row_counter % 2 == 1 {
                            result += 1;
                        }
                    }
                }
            } else if curr_row_counter % 2 == 1 {
                result += 1;
            }
        }
        // println!("Result: {}", result);
    }
    result
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day10a(input));
    println!("{}", day10b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input1() -> &'static str {
        ".....
.S-7.
.|.|.
.L-J.
....."
    }

    fn input2() -> &'static str {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
    }

    #[test]
    fn test_10a() {
        let input = input1();
        assert_eq!(day10a(input), 4);

        let input = input2();
        assert_eq!(day10a(input), 8);
    }

    fn input3() -> &'static str {
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
    }

    fn input4() -> &'static str {
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
    }

    fn input5() -> &'static str {
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
    }

    fn input6() -> &'static str {
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
    }

    #[test]
    fn test_10b() {
        let input = input3();
        assert_eq!(day10b(input), 4);

        let input = input4();
        assert_eq!(day10b(input), 4);

        let input = input5();
        assert_eq!(day10b(input), 8);

        let input = input6();
        assert_eq!(day10b(input), 10);
    }
}
