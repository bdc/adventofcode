use itertools::Itertools;

use crate::util::grid;
use std::{
    collections::{HashMap, VecDeque},
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

fn get_pos(grid: &HashMap<(i32, i32), char>, c: char) -> (i32, i32) {
    *grid.iter().find(|(_, &v)| v == c).unwrap().0
}

fn get_neighbors(pos: (i32, i32)) -> [(i32, i32); 4] {
    [
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ]
}

fn populate_costs(grid: &HashMap<(i32, i32), char>, start: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let mut costs: HashMap<(i32, i32), i32> = HashMap::from([(start, 0)]);
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        for neighbor in get_neighbors(pos) {
            if !grid.contains_key(&neighbor) {
                continue;
            }
            if grid[&neighbor] == '#' {
                continue;
            }
            if costs.contains_key(&neighbor) {
                continue;
            }
            costs.insert(neighbor, costs[&pos] + 1);
            queue.push_back(neighbor);
        }
    }
    costs
}

fn score_hack(costs: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> i32 {
    let neighbor_costs: Vec<i32> = get_neighbors(pos)
        .iter()
        .filter(|p| costs.contains_key(p))
        .map(|p| *costs.get(p).unwrap())
        .collect();
    if neighbor_costs.len() == 0 {
        return 0;
    }
    let min = *neighbor_costs.iter().min().unwrap();
    let max = *neighbor_costs.iter().max().unwrap();
    max - min - 2
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn score_savings(costs: &HashMap<(i32, i32), i32>, p1: (i32, i32), p2: (i32, i32)) -> i32 {
    if !costs.contains_key(&p1) || !costs.contains_key(&p2) {
        return 0;
    }
    let c1 = *costs.get(&p1).unwrap();
    let c2 = *costs.get(&p2).unwrap();
    let d = manhattan_distance(p1, p2);
    if d > 20 {
        return 0;
    }
    let savings = c2 - c1 - d;
    if savings < 0 {
        return 0;
    }
    savings
}

fn part1(input: &HashMap<(i32, i32), char>) -> usize {
    let start = get_pos(&input, 'S');
    let threshold = 100;
    let costs = populate_costs(&input, start);
    let scores = input
        .iter()
        .filter(|(_, &v)| v == '#')
        .map(|(&k, _)| score_hack(&costs, k))
        .sorted_by_key(|&v| v)
        .collect_vec();
    scores.iter().filter(|&v| *v >= threshold).count()
}

fn part2(input: &HashMap<(i32, i32), char>) -> usize {
    let start = get_pos(&input, 'S');
    let threshold = 100;
    let mut count = 0;
    let costs = populate_costs(&input, start);
    for &p1 in costs.keys() {
        for &p2 in costs.keys() {
            let savings = score_savings(&costs, p1, p2);
            if savings >= threshold {
                count += 1;
            }
        }
    }
    count
}
