use std::{
    cmp::max,
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

    let parsed = parse_input(&input);
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

struct Code {
    sequence: Vec<char>,
}

fn parse_input(input: &str) -> Vec<Code> {
    let mut codes = vec![];
    for line in input.lines() {
        codes.push(Code {
            sequence: line.trim().chars().collect(),
        });
    }
    codes
}

fn get_numeric_keypad_map() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ])
}

fn get_directional_keypad_map() -> HashMap<char, (i32, i32)> {
    HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ])
}

fn sequence_to_chunk_map(sequence: &Vec<char>) -> HashMap<String, usize> {
    let mut chunk_map = HashMap::new();
    let mut ptr = 0;
    for (i, &c) in sequence.iter().enumerate() {
        if c == 'A' {
            let vec_char_str = sequence[ptr..i + 1].iter().collect::<String>();
            let count = chunk_map.entry(vec_char_str).or_insert(0);
            *count += 1;
            ptr = i + 1;
        }
    }
    chunk_map
}

fn transform_chunk_map(
    chunk_map: &HashMap<String, usize>,
    directional_keypad_map: &HashMap<char, (i32, i32)>,
) -> HashMap<String, usize> {
    let mut new_chunk_map = HashMap::new();
    for (chunk, count) in chunk_map {
        let transformed_chunk =
            transform_sequence(&chunk.chars().collect(), directional_keypad_map);
        let transformed_chunk_map = sequence_to_chunk_map(&transformed_chunk);
        for (transformed_chunk, transformed_count) in transformed_chunk_map {
            let new_count = new_chunk_map.entry(transformed_chunk).or_insert(0);
            *new_count += transformed_count * count;
        }
    }
    new_chunk_map
}

fn calculate_min_length(sequence: &Vec<char>, n_intermediates: usize) -> i64 {
    let numeric_keypad_map = get_numeric_keypad_map();
    let directional_keypad_map = get_directional_keypad_map();
    let sequence = transform_sequence(&sequence, &numeric_keypad_map);
    let mut chunk_map = sequence_to_chunk_map(&sequence);
    for _ in 0..n_intermediates {
        chunk_map = transform_chunk_map(&chunk_map, &directional_keypad_map);
    }
    chunk_map
        .iter()
        .map(|(k, v)| k.len() * v)
        .map(|x| x as i64)
        .sum::<i64>()
}

fn is_valid_sequence(sequence: &str, key_map: &HashMap<char, (i32, i32)>, from: char) -> bool {
    let valid_pos = key_map.values().cloned().collect::<HashSet<_>>();
    let mut pos = key_map[&from];
    for c in sequence.chars() {
        match c {
            '<' => {
                pos.1 -= 1;
            }
            '>' => {
                pos.1 += 1;
            }
            '^' => {
                pos.0 -= 1;
            }
            'v' => {
                pos.0 += 1;
            }
            _ => unreachable!(),
        }
        if !valid_pos.contains(&pos) {
            return false;
        }
    }
    true
}

fn transform_sequence(sequence: &Vec<char>, key_map: &HashMap<char, (i32, i32)>) -> Vec<char> {
    let mut movements: Vec<char> = vec![];
    for (i, _) in sequence.iter().enumerate() {
        let c1 = if i == 0 { 'A' } else { sequence[i - 1] };
        let c2 = sequence[i];
        let dx = key_map[&c2].1 - key_map[&c1].1;
        let dy = key_map[&c2].0 - key_map[&c1].0;
        let dir_l = "<".repeat(max(-dx, 0) as usize);
        let dir_d = "v".repeat(max(dy, 0) as usize);
        let dir_u = "^".repeat(max(-dy, 0) as usize);
        let dir_r = ">".repeat(max(dx, 0) as usize);
        let mut result = format!("{}{}{}{}", dir_l, dir_d, dir_u, dir_r);

        if !is_valid_sequence(&result, key_map, c1) {
            result = result.chars().rev().collect::<String>();
        }
        movements.extend(result.chars());
        movements.push('A');
    }
    movements
}

fn calculate_numeric_part(sequence: &Vec<char>) -> usize {
    let mut val = 0;
    for &c in sequence {
        if c == 'A' {
            break;
        }
        val = val * 10 + (c.to_digit(10).unwrap() as usize);
    }
    val
}

#[allow(unused)]
fn part1(input: &Vec<Code>) -> i64 {
    let mut score = 0;
    input
        .iter()
        .map(|code| {
            let min_length = calculate_min_length(&code.sequence, 2);
            let numeric = calculate_numeric_part(&code.sequence) as i64;
            min_length * numeric
        })
        .sum()
}

#[allow(unused)]
fn part2(input: &Vec<Code>) -> i64 {
    let mut score = 0;
    input
        .iter()
        .map(|code| {
            let min_length = calculate_min_length(&code.sequence, 25);
            let numeric = calculate_numeric_part(&code.sequence) as i64;
            min_length * numeric
        })
        .sum()
}
