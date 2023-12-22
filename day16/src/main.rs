#[derive(PartialEq, Debug, Copy, Clone)]
enum Dim {
    X,
    Y,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
enum Mirror {
    Straight(Dim),
    Angled(Direction),
    Empty,
}

#[derive(Debug)]
struct Beam(Dim, Direction);

impl Beam {
    fn switch_dim(&mut self) {
        match &self.0 {
            Dim::X => self.0 = Dim::Y,
            Dim::Y => self.0 = Dim::X,
        }
    }

    fn switch_direction(&mut self) {
        match &self.1 {
            Direction::Forward => self.1 = Direction::Backward,
            Direction::Backward => self.1 = Direction::Forward,
        }
    }

    fn move_space(&self, i: i32, j: i32) -> (i32, i32) {
        match self.0 {
            Dim::X => match self.1 {
                Direction::Forward => (i, j + 1),
                Direction::Backward => (i, j - 1),
            },
            Dim::Y => match self.1 {
                Direction::Forward => (i + 1, j),
                Direction::Backward => (i - 1, j),
            },
        }
    }

    fn reflect(
        &mut self,
        map: &Vec<Vec<Mirror>>,
        start_pos: &(i32, i32),
        beams: &mut Vec<Vec<Vec<Beam>>>,
    ) -> Vec<Vec<usize>> {
        let mut result = vec![vec![0; map[0].len()]; map.len()];
        let (mut i, mut j) = start_pos;
        while let Some(val) = check_valid_indices(map, &(i, j)) {
            println!("i: {}, j: {}, val: {:?}, {:?}", i, j, val, self);
            result[i as usize][j as usize] += 1;
            match val {
                Mirror::Empty => (),
                Mirror::Straight(dim) => {
                    if self.0 != *dim {
                        self.1 = Direction::Forward;
                        self.0 = *dim;
                        result = sum_vecs(result, self.reflect(map, &self.move_space(i, j)));
                        self.1 = Direction::Backward;
                        self.0 = *dim;
                    }
                }
                Mirror::Angled(direction) => {
                    self.switch_dim();
                    match direction {
                        Direction::Forward => self.switch_direction(),
                        Direction::Backward => (),
                    }
                }
            }
            (i, j) = self.move_space(i, j)
        }
        result
    }
}

fn check_valid_indices<'a, T>(matrix: &'a Vec<Vec<T>>, indices: &'a (i32, i32)) -> Option<&'a T> {
    if indices.0 < 0 || indices.1 < 0 {
        return None;
    }
    if let Some(row) = matrix.get(indices.0 as usize) {
        row.get(indices.1 as usize)
    } else {
        None
    }
}

fn sum_vecs(vec1: Vec<Vec<usize>>, vec2: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for (i, vec) in vec1.iter().enumerate() {
        let mut new_vec = vec![];
        for (j, val) in vec.iter().enumerate() {
            new_vec.push(val + vec2[i][j]);
        }
        result.push(new_vec);
    }
    result
}

fn parse_input(input: &str) -> Vec<Vec<Mirror>> {
    input
        .lines()
        .map(|rows| {
            rows.chars()
                .map(|c| match c {
                    '.' => Mirror::Empty,
                    '-' => Mirror::Straight(Dim::X),
                    '|' => Mirror::Straight(Dim::Y),
                    '/' => Mirror::Angled(Direction::Forward),
                    '\\' => Mirror::Angled(Direction::Backward),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn day16a(input: &str) -> i32 {
    let map = parse_input(input);
    let mut beam = Beam(Dim::X, Direction::Forward);
    let results = beam.reflect(&map, &(0, 0));
    println!("{:?}", results);
    results.iter().fold(0, |acc, vec| {
        acc + vec.iter().filter(|&&x| x > 0).count() as i32
    })
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day16a(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
    }

    #[test]
    fn test_xa() {
        let input = input();
        assert_eq!(day16a(input), 1);
    }
}
