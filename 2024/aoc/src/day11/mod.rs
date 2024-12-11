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
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Stone {
    val: u64,
}

fn parse_input(input: &str) -> HashMap<Stone, u64> {
    let mut map = HashMap::new();
    input
        .to_ascii_lowercase()
        .split(" ")
        .map(|s| Stone {
            val: s.parse().unwrap(),
        })
        .for_each(|stone| {
            *map.entry(stone).or_insert(0) += 1;
        });
    map
}

fn step(stones: &HashMap<Stone, u64>) -> HashMap<Stone, u64> {
    let mut new_stones = HashMap::new();
    stones.iter().for_each(
        |(stone, count)| match (stone.val, stone.val.to_string().len() % 2) {
            (0, _) => *new_stones.entry(Stone { val: 1 }).or_insert(0) += *count,
            (_, 0) => {
                let s = stone.val.to_string();
                let l = &s[0..s.len() / 2].parse::<u64>().unwrap();
                let r = &s[s.len() / 2..].parse::<u64>().unwrap();
                *new_stones.entry(Stone { val: *l }).or_insert(0) += *count;
                *new_stones.entry(Stone { val: *r }).or_insert(0) += *count;
            }
            _ => {
                *new_stones
                    .entry(Stone {
                        val: stone.val * 2024,
                    })
                    .or_insert(0) += *count
            }
        },
    );
    new_stones
}

fn step_n_and_sum(input: &HashMap<Stone, u64>, n: u64) -> u64 {
    let mut stones = input.clone();
    for _ in 0..n {
        stones = step(&stones);
    }
    stones.values().map(|v| *v).sum::<u64>()
}

#[allow(unused)]
fn part1(input: &HashMap<Stone, u64>) -> u64 {
    step_n_and_sum(input, 25)
}

#[allow(unused)]
fn part2(input: &HashMap<Stone, u64>) -> u64 {
    step_n_and_sum(input, 75)
}
