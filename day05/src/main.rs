use rayon::prelude::*;

#[derive(Debug, Clone)]
struct XtoYMaps(Vec<Map>);

#[derive(Debug, Clone)]
struct Map {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

fn parse_seeds(seeds_raw: &str) -> Vec<i64> {
    seeds_raw
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_maps(maps_raw: &str) -> Vec<XtoYMaps> {
    let mut x_to_y_maps: Vec<XtoYMaps> = Vec::new();
    for map in maps_raw.split("\n\n") {
        let map_content = map.split_once("\n").unwrap().1;
        let mut maps = Vec::new();
        for line in map_content.lines() {
            let split_line = line
                .splitn(3, " ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            maps.push(Map {
                dest_start: split_line[0],
                src_start: split_line[1],
                length: split_line[2],
            })
        }
        x_to_y_maps.push(XtoYMaps(maps))
    }
    return x_to_y_maps;
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<XtoYMaps>) {
    let (seeds_raw, maps_raw) = input.split_once("\n\n").unwrap();
    let seeds = parse_seeds(seeds_raw);
    let x_to_y_maps = parse_maps(maps_raw);
    return (seeds, x_to_y_maps);
}

fn day5a(input: &str) -> i64 {
    let (seeds, x_to_y_maps) = parse_input(input);
    return seeds
        .iter()
        .map(|seed_number| {
            let mut number = seed_number.clone();
            for maps in x_to_y_maps.iter() {
                for map in &maps.0 {
                    if number >= map.src_start && number < map.src_start + map.length {
                        number = map.dest_start + number - map.src_start;
                        break;
                    }
                }
            }
            return number;
        })
        .min()
        .unwrap();
}

fn day5b(input: &str) -> i64 {
    let (seeds, x_to_y_maps) = parse_input(input);

    let result = seeds
        .par_chunks(2)
        .map(|chunk| {
            let number = chunk[0];
            let range = chunk[1];
            let mut local_result = i64::MAX;

            for seed in number..number + range {
                let mut local_number = seed;

                for maps in x_to_y_maps.iter() {
                    for map in &maps.0 {
                        if local_number >= map.src_start
                            && local_number < map.src_start + map.length
                        {
                            local_number = map.dest_start + local_number - map.src_start;
                            break;
                        }
                    }
                }

                if local_number < local_result {
                    local_result = local_number;
                }
            }

            local_result
        })
        .reduce(|| i64::MAX, |a, b| if a < b { a } else { b });

    result
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", day5a(input));
    println!("{}", day5b(input));
}

#[cfg(test)]
mod tests {
    use crate::day5a;
    use crate::day5b;

    fn input() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }

    #[test]
    fn test_5a() {
        let input = input();
        assert_eq!(day5a(input), 35);
    }

    #[test]
    fn test_5b() {
        let input = input();
        assert_eq!(day5b(input), 46);
    }
}
