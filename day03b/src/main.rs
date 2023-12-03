use std::collections::{hash_map::Entry, HashMap};

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut gearmap: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    fn update_gearmap(
        input: &Vec<&str>,
        row_ix: usize,
        col_min_ix: usize,
        col_max_ix: usize,
        num_str: &str,
        gearmap: &mut HashMap<(usize, usize), Vec<u32>>,
    ) {
        let min_i = (row_ix as i32 - 1).max(0) as usize;
        let max_i = (row_ix + 1).min(input.len() - 1);
        let min_j = (col_min_ix as i32 - 1).max(0) as usize;
        let max_j = (col_max_ix + 1).min(input.get(row_ix).unwrap().len() - 1);
        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let val = input.get(i).unwrap().chars().nth(j).unwrap();
                if val == '*' {
                    match gearmap.entry((i, j)) {
                        Entry::Vacant(e) => {
                            e.insert(vec![num_str.parse::<u32>().unwrap()]);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(num_str.parse::<u32>().unwrap());
                        }
                    }
                }
            }
        }
    }

    let mut num_str = String::new();
    let mut i = 0;
    for line in &input {
        if (num_str.len() > 0) & (i > 0) {
            let prev_line = *input.get(i - 1).unwrap();
            update_gearmap(
                &input,
                i - 1,
                prev_line.len() - num_str.len(),
                prev_line.len() - 1,
                &num_str,
                &mut gearmap,
            )
        }
        num_str = String::new();
        for j in 0..line.len() {
            let val = line.chars().nth(j).unwrap();
            if val.is_digit(10) {
                num_str.push(val);
            } else {
                if num_str.len() > 0 {
                    update_gearmap(&input, i, j - num_str.len(), j - 1, &num_str, &mut gearmap);
                    num_str = String::new();
                }
            }
        }
        i += 1
    }
    if num_str.len() > 0 {
        let prev_line = input.get(i - 1).unwrap();
        update_gearmap(
            &input,
            i - 1,
            prev_line.len() - num_str.len(),
            prev_line.len() - 1,
            &num_str,
            &mut gearmap,
        )
    }
    let mut result = 0;
    for (_, v) in gearmap {
        if v.len() == 2 {
            result += v.iter().product::<u32>();
        }
    }
    println!("{}", result)
}
