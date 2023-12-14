use num::integer::lcm;
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

fn num_steps(
    instructions: &str,
    network: &HashMap<&str, Vec<&str>>,
    start: &str,
    end: Vec<&str>,
) -> i64 {
    let mut current_node = start;
    let mut instruction_index = 0;
    let mut result = 0;
    while !end.contains(&current_node) {
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

fn day8a(input: &str) -> i32 {
    let (instructions, network) = parse_input(input);
    num_steps(instructions, &network, "AAA", vec!["ZZZ"].to_vec()) as i32
}

fn day8b(input: &str) -> i64 {
    let (instructions, network) = parse_input(input);
    let start_nodes = network
        .keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&k| k)
        .collect::<Vec<&str>>();

    let end_nodes = network
        .keys()
        .filter(|&k| k.ends_with('Z'))
        .map(|&k| k)
        .collect::<Vec<&str>>();

    let mut factors = Vec::new();
    for node in start_nodes.iter() {
        factors.push(num_steps(instructions, &network, node, end_nodes.clone()));
    }
    let mut result = 1 as i64;
    for factor in factors.iter() {
        result = lcm(result, *factor);
    }
    result
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
