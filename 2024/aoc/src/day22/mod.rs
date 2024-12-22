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

fn parse_input(input: &str) -> Vec<Secret> {
    input.lines().map(|line| Secret::new(line.trim())).collect()
}

#[derive(Debug, Clone)]
struct Secret {
    prev: i64,
    val: i64,
    deltas: [i64; 4],
}

impl Secret {
    fn new(str: &str) -> Self {
        Self {
            prev: 0,
            val: str.parse::<i64>().unwrap(),
            deltas: [99; 4],
        }
    }

    fn mix(&mut self, operand: i64) {
        self.val ^= operand;
    }

    fn prune(&mut self) {
        self.val %= 16777216
    }

    fn mult(&mut self, operand: i64) {
        self.mix(self.val * operand);
        self.prune();
    }

    fn div(&mut self, operand: i64) {
        self.mix(self.val / operand);
        self.prune();
    }

    fn handle_deltas(&mut self, values_map: &mut HashMap<(i64, i64, i64, i64), i64>) {
        self.deltas = [
            self.deltas[1],
            self.deltas[2],
            self.deltas[3],
            self.val % 10 - self.prev % 10,
        ];
        let key = (
            self.deltas[0],
            self.deltas[1],
            self.deltas[2],
            self.deltas[3],
        );
        if !values_map.contains_key(&key) {
            values_map.insert(key, self.val % 10);
        }
    }

    fn next(&mut self) {
        self.mult(64);
        self.div(32);
        self.mult(2048);
    }

    fn next_2(&mut self, values_map: &mut HashMap<(i64, i64, i64, i64), i64>) {
        self.prev = self.val;
        self.mult(64);
        self.div(32);
        self.mult(2048);
        self.handle_deltas(values_map);
    }
}

#[allow(unused)]
fn part1(input: &Vec<Secret>) -> i64 {
    input
        .iter()
        .map(|secret| secret.clone())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret.next();
            }
            secret.val
        })
        .sum()
}

#[allow(unused)]
fn part2(input: &Vec<Secret>) -> i64 {
    let mut values_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    input
        .iter()
        .map(|secret| secret.clone())
        .for_each(|mut secret| {
            let mut new_values_map = HashMap::new();
            for _ in 0..2000 {
                secret.next_2(&mut new_values_map);
            }
            new_values_map.iter().for_each(|(key, value)| {
                *values_map.entry(*key).or_insert(0) += *value;
            });
        });
    *values_map.values().max().unwrap()
}
