use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize, usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    adj_list: &Vec<Vec<Vec<Vec<Vec<(usize, usize, usize, usize, u32)>>>>>,
    start: (usize, usize, usize, usize),
    goals: Vec<(usize, usize, usize, usize)>,
) -> Option<u32> {
    let mut dist: Vec<Vec<Vec<Vec<u32>>>> =
        vec![
            vec![vec![vec![u32::MAX; adj_list[0][0][0].len()]; 4]; adj_list[0].len()];
            adj_list.len()
        ];
    let mut heap = BinaryHeap::new();

    dist[start.0][start.1][start.2][start.3] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if goals.contains(&position) {
            return Some(cost);
        }

        if cost > dist[position.0][position.1][position.2][position.3] {
            continue;
        }

        for node_pos in &adj_list[position.0][position.1][position.2][position.3] {
            let next = State {
                cost: node_pos.4 + cost,
                position: (node_pos.0, node_pos.1, node_pos.2, node_pos.3),
            };

            if next.cost < dist[next.position.0][next.position.1][next.position.2][next.position.3]
            {
                heap.push(next);
                dist[next.position.0][next.position.1][next.position.2][next.position.3] =
                    next.cost;
            }
        }
    }
    None
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn eligible_directions(
    prev_direction: Direction,
    blocks_moved: usize,
    min_same_dir: usize,
    max_same_dir: usize,
) -> Vec<Direction> {
    if blocks_moved < min_same_dir {
        return vec![prev_direction];
    }
    let mut eligible_directions = vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
    let opposite_direction = match prev_direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    };

    eligible_directions.retain(|d| *d != opposite_direction);

    if blocks_moved >= max_same_dir {
        eligible_directions.retain(|d| *d != prev_direction);
    }

    eligible_directions
}

fn move_in_direction(
    row_index: usize,
    col_index: usize,
    direction: &Direction,
    blocks: usize,
    map: &Vec<Vec<u32>>,
) -> Option<(usize, usize, u32)> {
    match direction {
        Direction::Up if row_index >= blocks => {
            return Some((row_index - 1, col_index, map[row_index - 1][col_index]));
        }
        Direction::Down if row_index + blocks < map.len() => {
            return Some((row_index + 1, col_index, map[row_index + 1][col_index]));
        }
        Direction::Left if col_index >= blocks => {
            return Some((row_index, col_index - 1, map[row_index][col_index - 1]));
        }
        Direction::Right if col_index + blocks < map[row_index].len() => {
            return Some((row_index, col_index + 1, map[row_index][col_index + 1]));
        }
        _ => return None,
    }
}

fn find_adjacent_nodes(
    row_index: usize,
    col_index: usize,
    prev_direction: Direction,
    blocks_moved: usize,
    map: &Vec<Vec<u32>>,
    min_same_dir: usize,
    max_same_dir: usize,
) -> Vec<(usize, usize, usize, usize, u32)> {
    let mut adjacent_nodes = Vec::new();
    let eligible_directions =
        eligible_directions(prev_direction, blocks_moved, min_same_dir, max_same_dir);
    for direction in eligible_directions {
        if let Some((new_row_index, new_col_index, value)) =
            move_in_direction(row_index, col_index, &direction, 1, map)
        {
            let mut new_blocks_moved = 1;
            if direction == prev_direction {
                new_blocks_moved += blocks_moved;
            }
            adjacent_nodes.push((
                new_row_index,
                new_col_index,
                direction as usize,
                new_blocks_moved,
                value,
            ))
        }
    }
    adjacent_nodes
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>()
}

fn adjacency_matrix(
    map: &Vec<Vec<u32>>,
    min_same_dir: usize,
    max_same_dir: usize,
) -> Vec<Vec<Vec<Vec<Vec<(usize, usize, usize, usize, u32)>>>>> {
    let mut adj_mat = vec![vec![vec![vec![vec![]; max_same_dir + 1]; 4]; map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for l in 0..=max_same_dir {
                    adj_mat[i][j][direction as usize][l] =
                        find_adjacent_nodes(i, j, direction, l, &map, min_same_dir, max_same_dir);
                }
            }
        }
    }
    adj_mat
}

fn day17(input: &str, min_same_dir: usize, max_same_dir: usize) -> Option<u32> {
    let map = parse_input(input);
    let adj_mat = adjacency_matrix(&map, min_same_dir, max_same_dir);

    let start_node = (0, 0, Direction::Right as usize, 0);
    let mut end_nodes = Vec::new();
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        for l in min_same_dir..=max_same_dir {
            end_nodes.push((map.len() - 1, map[0].len() - 1, direction as usize, l));
        }
    }
    shortest_path(&adj_mat, start_node, end_nodes)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", day17(input, 0, 3));
    println!("{:?}", day17(input, 4, 10));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    }

    #[test]
    fn test_17a() {
        let input = input();
        assert_eq!(day17(input, 0, 3), Some(102));
    }
    #[test]
    fn test_17b() {
        let input = input();
        assert_eq!(day17(input, 4, 10), Some(94));
    }
}
