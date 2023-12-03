use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                let part2 = line.split_once(": ").unwrap().1;
                let mut colormaxes: HashMap<&str, i32> =
                    HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
                part2.split("; ").for_each(|set| {
                    set.split(", ").for_each(|cubes| {
                        let (n, color) = cubes.split_once(" ").unwrap();
                        let n_int = n.parse::<i32>().unwrap();
                        if n_int > *colormaxes.get(color).unwrap() {
                            colormaxes.insert(color, n_int);
                        }
                    })
                });
                return colormaxes.values().product::<i32>();
            })
            .sum::<i32>()
    )
}
