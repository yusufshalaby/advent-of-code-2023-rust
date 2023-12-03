fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    fn check_num(input: &Vec<&str>, row_ix: usize, col_min_ix: usize, col_max_ix: usize) -> bool {
        let min_i = (row_ix as i32 - 1).max(0) as usize;
        let max_i = (row_ix + 1).min(input.len() - 1);
        let min_j = (col_min_ix as i32 - 1).max(0) as usize;
        let max_j = (col_max_ix + 1).min(input.get(row_ix).unwrap().len() - 1);
        for i in min_i..max_i + 1 {
            for j in min_j..max_j + 1 {
                let val = input.get(i).unwrap().chars().nth(j).unwrap();
                if (!val.is_digit(10)) & (val != '.') {
                    return true;
                }
            }
        }
        false
    }

    let mut result = 0;
    let mut num_str = String::new();
    let mut i = 0;
    for line in &input {
        if (num_str.len() > 0) & (i > 0) {
            let prev_line = *input.get(i - 1).unwrap();
            if check_num(
                &input,
                i - 1,
                prev_line.len() - num_str.len(),
                prev_line.len() - 1,
            ) {
                result += num_str.parse::<i32>().unwrap()
            }
        }
        num_str = String::new();
        for j in 0..line.len() {
            let val = line.chars().nth(j).unwrap();
            if val.is_digit(10) {
                num_str.push(val);
            } else {
                if num_str.len() > 0 {
                    if check_num(&input, i, j - num_str.len(), j - 1) {
                        result += num_str.parse::<i32>().unwrap()
                    }
                    num_str = String::new();
                }
            }
        }
        i += 1
    }
    if num_str.len() > 0 {
        let prev_line = input.get(i - 1).unwrap();
        if check_num(
            &input,
            i - 1,
            prev_line.len() - num_str.len(),
            prev_line.len() - 1,
        ) {
            result += num_str.parse::<i32>().unwrap()
        }
    }
    println!("{}", result)
}
