#![warn(clippy::pedantic)]
#![allow(dead_code)]

use std::collections::HashMap;
#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Condition {
    category: Category,
    operator: Operator,
    value: i32,
    destination: String,
}

#[derive(Debug)]
enum Rule {
    Condition(Condition),
    Destination(String),
}

fn find_destination(workflows: &HashMap<String, Vec<Rule>>, part: &Part, source: &str) -> String {
    workflows
        .get(source)
        .and_then(|rules| {
            rules.iter().find_map(|rule| match rule {
                Rule::Condition(condition) => {
                    let value = match condition.category {
                        Category::X => part.x,
                        Category::M => part.m,
                        Category::A => part.a,
                        Category::S => part.s,
                    };
                    let result = match condition.operator {
                        Operator::LessThan => value < condition.value,
                        Operator::GreaterThan => value > condition.value,
                    };
                    if result && (condition.destination == "A" || condition.destination == "R") {
                        Some(condition.destination.clone())
                    } else if result {
                        Some(find_destination(workflows, part, &condition.destination))
                    } else {
                        None
                    }
                }
                Rule::Destination(dest) if dest == "A" || dest == "R" => Some(dest.clone()),
                Rule::Destination(dest) => Some(find_destination(workflows, part, dest)),
            })
        })
        .unwrap_or_default()
}

fn parse_workflows(input: &str) -> HashMap<String, Vec<Rule>> {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    for line in input.lines() {
        let (code, workflow) = line.split_once('{').expect("failed to parse workflow");
        let workflow = workflow[..workflow.len() - 1].to_string();
        let workflow = workflow.split(',').map(|rule| {
            if let Some((comp, dest)) = rule.split_once(':') {
                let mut comp = comp.chars();
                let category = match comp.next().expect("failed to parse category") {
                    'x' => Category::X,
                    'm' => Category::M,
                    'a' => Category::A,
                    's' => Category::S,
                    _ => panic!("invalid category"),
                };
                let operator = match comp.next().expect("failed to parse operator") {
                    '>' => Operator::GreaterThan,
                    '<' => Operator::LessThan,
                    _ => panic!("invalid operator"),
                };
                let value = comp
                    .collect::<String>()
                    .parse::<i32>()
                    .expect("failed to parse comp");
                Rule::Condition(Condition {
                    category,
                    operator,
                    value,
                    destination: dest.to_string(),
                })
            } else {
                Rule::Destination(rule.to_string())
            }
        });
        workflows.insert(code.to_string(), workflow.collect());
    }
    // add the two final destinations
    workflows
}

fn parse_parts(input: &str) -> Vec<Part> {
    let result: Vec<Part> = input
        .lines()
        .map(|line| {
            let mut iter = line[1..line.len() - 1].splitn(4, ',').map(|part| {
                let (_, value) = part.split_once('=').unwrap();
                value.parse::<i32>().expect("failed to parse part")
            });

            Part {
                x: iter.next().expect("failed to parse 'x'"),
                m: iter.next().expect("failed to parse 'm'"),
                a: iter.next().expect("failed to parse 'a'"),
                s: iter.next().expect("failed to parse 's'"),
            }
        })
        .collect();

    result
}

fn day19a(input: &str) -> i32 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let parts = parse_parts(parts);
    let mut result = 0;
    let source: &str = "in";
    for part in parts {
        let destination = find_destination(&workflows, &part, source);
        println!("{part:?} {source} -> {destination}");
        if destination == "A" {
            result += part.x + part.m + part.a + part.s;
        }
    }
    result
}

fn day19b(_input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day19a(input));
    println!("{}", day19b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
    }

    #[test]
    fn test_19a() {
        let input = input();
        assert_eq!(day19a(input), 19114);
    }

    #[test]
    fn test_xb() {
        let input = input();
        assert_eq!(day19b(input), 0);
    }
}
