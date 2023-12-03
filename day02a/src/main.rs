use std::collections::HashMap;

fn main() {
    let colormaxes: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let (part1, part2) = line.split_once(": ").unwrap();
                if part2.split("; ").all(|set| {
                    set.split(", ").all(|cubes| {
                        let (n, color) = cubes.split_once(" ").unwrap();
                        if n.parse::<i32>().unwrap() > *colormaxes.get(color).unwrap() {
                            return false;
                        }
                        true
                    })
                }) {
                    return part1.split_once(" ").unwrap().1.parse::<u32>().unwrap();
                }
                0
            })
            .sum::<u32>()
    )
}
