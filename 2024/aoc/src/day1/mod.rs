use std::{collections::HashMap, fs};

const DEFAULT_INPUT_FILE: &str = "input.txt";

pub fn main(_args: Vec<String>) {
    let this_file = file!();
    let this_dir = std::path::Path::new(this_file).parent().unwrap();
    let mut file_name = DEFAULT_INPUT_FILE;
    if _args.len() >= 1 {
        file_name = &_args[0];
    }
    let input = fs::read_to_string(this_dir.join(file_name)).unwrap();

    let parsed = parse_input(&input);
    let result = part1(parsed.clone());
    println!("Part 1: {}", result);
    let result = part2(parsed.clone());
    println!("Part 2: {}", result);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut lefts: Vec<i32> = input.iter().map(|v| v[0]).collect();
    let mut rights: Vec<i32> = input.iter().map(|v| v[1]).collect();
    lefts.sort();
    rights.sort();
    let diffs: Vec<i32> = lefts
        .iter()
        .zip(rights.iter())
        .map(|(l, r)| (r - l).abs())
        .collect();
    diffs.iter().sum::<i32>()
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let lefts: Vec<i32> = input.iter().map(|v| v[0]).collect();
    let rights: Vec<i32> = input.iter().map(|v| v[1]).collect();
    let left_map = list_to_map(lefts);
    let right_map = list_to_map(rights);
    left_map.iter().fold(0, |acc, (k, v)| {
        acc + k * v * right_map.get(k).unwrap_or(&0)
    })
}

fn list_to_map(list: Vec<i32>) -> HashMap<i32, i32> {
    list.iter().fold(HashMap::new(), |mut map, &l| {
        *map.entry(l).or_insert(0) += 1;
        map
    })
}
