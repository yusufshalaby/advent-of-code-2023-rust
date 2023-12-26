use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn eligible_directions(prev_direction: Direction, blocks_moved: usize) -> Vec<Direction> {
    if blocks_moved < 4 {
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

    if blocks_moved >= 10 {
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
    let mut blocks_traversed = Vec::new();
    match direction {
        Direction::Up if row_index >= blocks => {
            for i in 1..=blocks {
                blocks_traversed.push((row_index - i, col_index));
            }
        }
        Direction::Down if row_index + blocks < map.len() => {
            for i in 1..=blocks {
                blocks_traversed.push((row_index + i, col_index));
            }
        }
        Direction::Left if col_index >= blocks => {
            for i in 1..=blocks {
                blocks_traversed.push((row_index, col_index - i));
            }
        }
        Direction::Right if col_index + blocks < map[row_index].len() => {
            for i in 1..=blocks {
                blocks_traversed.push((row_index, col_index + i));
            }
        }
        _ => return None,
    };

    let mut value = 0;
    for (row_index, col_index) in blocks_traversed.iter() {
        value += map[*row_index][*col_index];
    }
    let (new_row_index, new_col_index) = blocks_traversed.last()?;

    Some((*new_row_index, *new_col_index, value))
}

fn find_adjacent_nodes(
    row_index: usize,
    col_index: usize,
    prev_direction: Direction,
    blocks_moved: usize,
    map: &Vec<Vec<u32>>,
) -> Vec<(usize, usize, usize, usize, u32)> {
    let mut adjacent_nodes = Vec::new();
    let eligible_directions = eligible_directions(prev_direction, blocks_moved);
    // add edge between curr node and one block in all eligible directions
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

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize, usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // .then_with(|| other.position.3.cmp(&self.position.3))
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
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<Vec<Vec<u32>>>> =
        vec![vec![vec![vec![u32::MAX; 11]; 4]; adj_list[0].len()]; adj_list.len()];
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.0][start.1][start.2][start.3] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        position,
    }) = heap.pop()
    {
        if goals.contains(&position) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.0][position.1][position.2][position.3] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node_pos in &adj_list[position.0][position.1][position.2][position.3] {
            let next = State {
                cost: node_pos.4 + cost,
                position: (node_pos.0, node_pos.1, node_pos.2, node_pos.3),
            };

            // If so, add it to the frontier and continue
            // println!("{:?}", next);
            if next.cost < dist[next.position.0][next.position.1][next.position.2][next.position.3]
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.0][next.position.1][next.position.2][next.position.3] =
                    next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn day17b(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    // dimensions: row, col, direction, blocks moved, adjacent nodes
    let mut dir_vecs: Vec<Vec<Vec<Vec<Vec<(usize, usize, usize, usize, u32)>>>>> =
        vec![vec![vec![vec![vec![]; 11]; 4]; map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // directions (up, down, left, right)
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                // blocks moved (0, 1, 2, 3)
                for l in 0..11 {
                    dir_vecs[i][j][direction as usize][l] =
                        find_adjacent_nodes(i, j, direction, l, &map);
                }
            }
        }
    }
    let mut end_nodes = Vec::new();
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        for l in 4..10 {
            end_nodes.push((map.len() - 1, map[0].len() - 1, direction as usize, l));
        }
    }
    shortest_path(&dir_vecs, (0, 0, Direction::Right as usize, 0), end_nodes)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", day17b(input));
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
    fn test_17b() {
        let input = input();
        assert_eq!(day17b(input), Some(94));
    }
}
