fn hashfn(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, b| ((acc + *b as usize) * 17) % 256)
}

fn day15a(input: &str) -> usize {
    input
        .replace("\n", "")
        .split(",")
        .fold(0, |acc, s| acc + hashfn(s))
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Lense {
    label: String,
    focal_point: u8,
}

fn day15b(input: &str) -> usize {
    let mut hashmap: Vec<Vec<Lense>> = vec![vec![]; 256];
    for step in input.replace("\n", "").split(",") {
        if step.chars().last().unwrap() == '-' {
            let label = &step[..step.len() - 1];
            let hash = hashfn(&label);
            hashmap[hash].retain(|lense| lense.label != label);
        } else {
            let (label, focal_point) = step.split_once("=").unwrap();
            let focal_point = focal_point.parse::<u8>().unwrap();
            let hash = hashfn(&label);
            let mut found_label = false;
            for i in 0..hashmap[hash].len() {
                if hashmap[hash][i].label == label {
                    hashmap[hash][i].focal_point = focal_point;
                    found_label = true;
                    break;
                }
            }
            if !found_label {
                hashmap[hash].push(Lense {
                    label: label.to_string(),
                    focal_point,
                })
            }
        }
    }
    hashmap.iter().enumerate().fold(0, |acc, (i, lense)| {
        acc + lense.iter().enumerate().fold(0, |acc, (j, lense)| {
            acc + (i + 1) * (j + 1) * lense.focal_point as usize
        })
    })
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day15a(input));
    println!("{}", day15b(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn input() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_xa() {
        let input = input();
        assert_eq!(day15a(input), 1320);
    }
    #[test]
    fn test_xb() {
        let input = input();
        assert_eq!(day15b(input), 145);
    }
}
