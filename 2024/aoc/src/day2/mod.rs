use std::fs;

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

fn is_safe(line: &Vec<i32>) -> bool {
    let diffs = line
        .iter()
        .zip(line.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<i32>>();
    let is_increasing = diffs.iter().all(|d| d > &0);
    let is_decreasing = diffs.iter().all(|d| d < &0);
    let is_diffs_ge_1 = diffs.iter().all(|d| d.abs() >= 1);
    let is_diffs_le_3 = diffs.iter().all(|d| d.abs() <= 3);
    (is_increasing || is_decreasing) && is_diffs_ge_1 && is_diffs_le_3
}

fn part1(input: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in input {
        if is_safe(&line) {
            sum += 1;
        }
    }
    sum
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in input {
        for i in 0..line.len() {
            let new_line = line
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, x)| *x)
                .collect::<Vec<i32>>();
            if is_safe(&new_line) {
                sum += 1;
                break;
            }
        }
    }
    sum
}
