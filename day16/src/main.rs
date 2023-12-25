#[derive(PartialEq, Debug, Copy, Clone)]
enum Dim {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
struct Beam(Dim, Direction);

struct State {
    beam: Beam,
    pos: (i32, i32),
    beams: Vec<Vec<Vec<Beam>>>,
}

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

}
impl State {
    
    fn new(beam: Beam, pos: (i32, i32), beams: Vec<Vec<Vec<Beam>>>) -> Self {
        Self { beam, pos, beams }
    }

    fn move_space(&mut self){
        match self.beam.0 {
            Dim::X => match self.beam.1 {
                Direction::Forward => self.pos.1 += 1,
                Direction::Backward => self.pos.1 -= 1,
            },
            Dim::Y => match self.beam.1 {
                Direction::Forward => self.pos.0 += 1,
                Direction::Backward => self.pos.0 -= 1,
            },
        }
    }

    fn reflect<'a>(
        &'a mut self,
        map: &Vec<Vec<Mirror>>,
    ) {
        while let Some(val) = self.check_valid_indices(map) {
            self.beams[self.pos.0 as usize][self.pos.1 as usize].push(self.beam.clone());
            match val {
                Mirror::Empty => (),
                Mirror::Straight(dim) => {
                    if self.beam.0 != *dim {
                        self.beam = Beam(*dim, Direction::Forward);
                        let (i, j) = self.pos.clone();
                        self.reflect(map);
                        self.pos = (i, j);
                        self.beam = Beam(*dim, Direction::Backward);
                    }
                }
                Mirror::Angled(direction) => {
                    self.beam.switch_dim();
                    match direction {
                        Direction::Forward => self.beam.switch_direction(),
                        Direction::Backward => (),
                    }
                }
            }
            self.move_space();
        }
    }
    fn check_valid_indices<'a>(
        &self,
        map: &'a Vec<Vec<Mirror>>,
    ) -> Option<&'a Mirror> {
        if self.pos.0 < 0 || self.pos.1 < 0 {
            return None;
        }
        map
            .get(self.pos.0 as usize)
            .and_then(|row| row.get(self.pos.1 as usize))
            .filter(|_| {
                self.beams[self.pos.0 as usize][self.pos.1 as usize]
                    .iter()
                    .all(|beam| *beam != self.beam)
            })
    }
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
    let beam = Beam(Dim::X, Direction::Forward);
    let beams = vec![vec![vec![]; map[0].len()]; map.len()];
    let mut state = State::new(beam, (0, 0), beams);
    state.reflect(&map);
    state.beams.iter().fold(0, |acc, vec| {
        acc + vec.iter().filter(|&x| x.len() > 0).count() as i32
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
        assert_eq!(day16a(input), 46);
    }
}
