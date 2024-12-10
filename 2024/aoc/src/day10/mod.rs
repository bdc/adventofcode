use crate::util::grid;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const DEFAULT_INPUT_FILE: &str = "input.txt";

pub fn main(_args: Vec<String>) {
    let this_file = file!();
    let this_dir = std::path::Path::new(this_file).parent().unwrap();
    let mut file_name = DEFAULT_INPUT_FILE;
    if _args.len() >= 1 {
        file_name = &_args[0];
    }
    let input = fs::read_to_string(this_dir.join(file_name)).unwrap();

    let parsed = grid::parse_input_to_grid(&input);
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

fn generate_waypoint_map(
    input: &HashMap<(i32, i32), char>,
) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    let mut map: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    // insert 9s
    for ((i, j), v) in input {
        if *v == '9' {
            map.insert((*i, *j), HashSet::from([(*i, *j)]));
        }
    }

    for n in (0..=8).rev() {
        let n_char = (n + '0' as u8) as char;
        let np1_char = (n + 1 + '0' as u8) as char;
        let nodes = input
            .iter()
            .filter(|(_, v)| **v == n_char)
            .map(|(k, _)| *k)
            .collect::<HashSet<(i32, i32)>>();
        // println!("{:?} {:?}", n, nodes);
        for (i, j) in nodes {
            let mut set_of_nines: HashSet<(i32, i32)> = HashSet::new();
            [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
                .iter()
                .filter(|(i2, j2)| map.contains_key(&(*i2, *j2)))
                .filter(|(i2, j2)| input.get(&(*i2, *j2)).unwrap() == &np1_char)
                .map(|(i2, j2)| (*i2, *j2))
                .for_each(|(i2, j2)| {
                    set_of_nines.extend(map.get(&(i2, j2)).unwrap());
                });
            map.insert((i, j), set_of_nines);
        }
    }

    map
}

fn generate_ratings_map(input: &HashMap<(i32, i32), char>) -> HashMap<(i32, i32), i32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    // insert 9s
    for ((i, j), v) in input {
        if *v == '9' {
            map.insert((*i, *j), 1);
        }
    }

    for n in (0..=8).rev() {
        let n_char = (n + '0' as u8) as char;
        let np1_char = (n + 1 + '0' as u8) as char;
        let nodes = input
            .iter()
            .filter(|(_, v)| **v == n_char)
            .map(|(k, _)| *k)
            .collect::<HashSet<(i32, i32)>>();
        // println!("{:?} {:?}", n, nodes);
        for (i, j) in nodes {
            let mut rating = 0;
            [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
                .iter()
                .filter(|(i2, j2)| map.contains_key(&(*i2, *j2)))
                .filter(|(i2, j2)| input.get(&(*i2, *j2)).unwrap() == &np1_char)
                .map(|(i2, j2)| (*i2, *j2))
                .for_each(|(i2, j2)| {
                    rating += map.get(&(i2, j2)).unwrap();
                });
            map.insert((i, j), rating);
        }
    }

    map
}

fn part1(input: &HashMap<(i32, i32), char>) -> i32 {
    // grid::print_grid(input);
    let map = generate_waypoint_map(input);
    input
        .iter()
        .filter(|(_, v)| **v == '0')
        .map(|(k, _)| map.get(k).unwrap().len() as i32)
        .sum()
}

fn part2(input: &HashMap<(i32, i32), char>) -> i32 {
    let map = generate_ratings_map(input);
    input
        .iter()
        .filter(|(_, v)| **v == '0')
        .map(|(k, _)| map.get(k).unwrap())
        .sum()
}
