use std::collections::HashMap;

fn parse_input(input: &str) -> (&str, HashMap<&str, Vec<&str>>) {
    let (instructions, network_raw) = input.split_once("\n\n").unwrap();
    let mut network = HashMap::new();
    for line in network_raw.lines() {
        let (node, children) = line.split_once(" = ").unwrap();
        let children: Vec<&str> = children
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .collect();
        network.insert(node, children);
    }
    (instructions, network)
}

fn day8a(input: &str) -> i32 {
    let (instructions, network) = parse_input(input);
    let mut current_node = "AAA";
    let mut instruction_index = 0;
    let mut result = 0;
    while current_node != "ZZZ" {
        match instructions.chars().nth(instruction_index) {
            Some('L') => current_node = network.get(current_node).unwrap()[0],
            Some('R') => current_node = network.get(current_node).unwrap()[1],
            _ => panic!("unknown direction!"),
        }
        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }
        result += 1;
    }
    result
}

fn day8b(input: &str) -> i32 {
    enum Direction {
        Left,
        Right,
    }

    fn update_current_nodes<'a>(
        current_nodes: &mut Vec<&'a str>,
        network: &HashMap<&str, Vec<&'a str>>,
        direction: Direction,
    ) {
        let mut next_current_nodes: Vec<&'a str> = Vec::new();
        for node in current_nodes.iter() {
            match direction {
                Direction::Left => next_current_nodes.push(network.get(node).unwrap()[0]),
                Direction::Right => next_current_nodes.push(network.get(node).unwrap()[1]),
            }
        }
        *current_nodes = next_current_nodes;
    }

    let (instructions, network) = parse_input(input);
    let mut current_nodes = network
        .keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&k| k)
        .collect::<Vec<&str>>();
    let mut result = 0;
    let mut instruction_index = 0;
    println!("{:?} \n", current_nodes);

    loop {
        match instructions.chars().nth(instruction_index) {
            Some('L') => update_current_nodes(&mut current_nodes, &network, Direction::Left),
            Some('R') => update_current_nodes(&mut current_nodes, &network, Direction::Right),
            _ => panic!("unknown direction!"),
        }
        result += 1;
        if current_nodes.iter().all(|&n| n.ends_with('Z')) {
            break result;
        }
        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }
        if result % 1e6 as i32 == 0 {
            println!("{:?}", result);
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day8a(input));
    println!("{}", day8b(input));
}

#[cfg(test)]
mod tests {
    use crate::day8a;
    use crate::day8b;

    fn input1() -> &'static str {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input2() -> &'static str {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input3() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }

    #[test]
    fn test_8a() {
        let input1 = input1();
        assert_eq!(day8a(input1), 2);

        let input2 = input2();
        assert_eq!(day8a(input2), 6);
    }

    #[test]
    fn test_8b() {
        let input3 = input3();
        assert_eq!(day8b(input3), 6);
    }
}
