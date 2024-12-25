use std::{collections::HashSet, fs};

use itertools::Itertools;

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
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Item {
    cols: Vec<usize>,
}

fn parse_item(lines: Vec<&str>) -> Item {
    let mut cols = vec![0; 5];
    for line in lines.iter().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                cols[i] += 1;
            }
        }
    }
    Item { cols }
}

fn parse_input(input: &str) -> (HashSet<Item>, HashSet<Item>) {
    let mut locks: HashSet<Item> = HashSet::new();
    let mut keys: HashSet<Item> = HashSet::new();

    input.lines().enumerate().step_by(8).for_each(|(i, line)| {
        let item_lines = input.lines().skip(i).take(6).collect_vec();
        let item: Item = parse_item(item_lines);
        if line.starts_with("#") {
            locks.insert(item);
        } else {
            keys.insert(item);
        }
    });
    (locks, keys)
}

fn fits(lock: &Item, key: &Item) -> bool {
    for i in 0..lock.cols.len() {
        if lock.cols[i] + key.cols[i] > 5 {
            return false;
        }
    }
    true
}

#[allow(unused)]
fn part1(input: &(HashSet<Item>, HashSet<Item>)) -> usize {
    let locks = input.0.clone();
    let keys = input.1.clone();
    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(lock, key) {
                count += 1;
            }
        }
    }
    count
}

#[allow(unused)]
fn part2(input: &(HashSet<Item>, HashSet<Item>)) -> usize {
    0
}
