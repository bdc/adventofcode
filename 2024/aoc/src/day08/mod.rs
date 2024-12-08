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

fn calculate_antinodes_v1(nodes: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    nodes.iter().for_each(|(i1, j1)| {
        nodes.iter().for_each(|(i2, j2)| {
            if i1 == i2 && j1 == j2 {
                return;
            }
            let i3 = i1 * 2 - i2;
            let j3 = j1 * 2 - j2;
            let i4 = i2 * 2 - i1;
            let j4 = j2 * 2 - j1;
            antinodes.insert((i3, j3));
            antinodes.insert((i4, j4));
        });
    });
    antinodes
}

fn calculate_antinodes_v2(
    nodes: &HashSet<(i32, i32)>,
    min_i: i32,
    max_i: i32,
    min_j: i32,
    max_j: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    nodes.iter().for_each(|(i1, j1)| {
        nodes.iter().for_each(|(i2, j2)| {
            if i1 == i2 && j1 == j2 {
                return;
            }
            let mut i = i2.clone();
            let mut j = j2.clone();
            while min_i <= i && i <= max_i && min_j <= j && j <= max_j {
                antinodes.insert((i, j));
                i = i + i2 - i1;
                j = j + j2 - j1;
            }
        });
    });
    antinodes
}

fn part1(input: &HashMap<(i32, i32), char>) -> i32 {
    // grid::print_grid(input);
    let mut antinodes = HashSet::<(i32, i32)>::new();
    let antenna_chars = input
        .values()
        .map(|c| *c)
        .filter(|c| c != &'.')
        .collect::<HashSet<_>>();
    antenna_chars.iter().for_each(|c| {
        let nodes: HashSet<(i32, i32)> = input
            .iter()
            .filter(|(_, v)| *v == c)
            .map(|(k, _)| *k)
            .collect::<HashSet<_>>();
        let new_antinodes = calculate_antinodes_v1(&nodes);
        antinodes.extend(new_antinodes);
    });
    antinodes
        .iter()
        .filter(|(i, j)| input.contains_key(&(*i, *j)))
        .count() as i32
}

fn part2(input: &HashMap<(i32, i32), char>) -> i32 {
    let mut antinodes = HashSet::<(i32, i32)>::new();

    let min_i = *input.keys().map(|(i, _)| i).min().unwrap();
    let max_i = *input.keys().map(|(i, _)| i).max().unwrap();
    let min_j = *input.keys().map(|(_, j)| j).min().unwrap();
    let max_j = *input.keys().map(|(_, j)| j).max().unwrap();

    let antenna_chars = input
        .values()
        .map(|c| *c)
        .filter(|c| c != &'.')
        .collect::<HashSet<_>>();
    antenna_chars.iter().for_each(|c| {
        let nodes: HashSet<(i32, i32)> = input
            .iter()
            .filter(|(_, v)| *v == c)
            .map(|(k, _)| *k)
            .collect::<HashSet<_>>();
        let new_antinodes = calculate_antinodes_v2(&nodes, min_i, max_i, min_j, max_j);
        antinodes.extend(new_antinodes);
    });
    antinodes
        .iter()
        .filter(|(i, j)| input.contains_key(&(*i, *j)))
        .count() as i32
}
