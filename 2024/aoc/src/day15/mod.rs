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

    let (grid, moves) = parse_input(&input);
    let result = part1(&grid, &moves);
    println!("Part 1: {}", result);
    let result = part2(&grid, &moves);
    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct Move {
    direction: char,
}

fn parse_input(input: &str) -> (HashMap<(i32, i32), char>, Vec<Move>) {
    let mut grid = HashMap::new();
    let mut moves = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if line.starts_with("#") {
            line.chars().enumerate().for_each(|(j, c)| {
                grid.insert((i as i32, j as i32), c);
            });
        } else {
            for c in line.trim().chars() {
                moves.push(Move { direction: c });
            }
        }
    });
    (grid, moves)
}

fn find_robot_pos(grid: &HashMap<(i32, i32), char>) -> (i32, i32) {
    grid.iter()
        .find(|(_, &c)| c == '@')
        .map(|(i, _)| *i)
        .unwrap()
}

fn move_char(grid: &mut HashMap<(i32, i32), char>, to: (i32, i32), from: (i32, i32)) -> char {
    let c = grid[&from];
    grid.insert(to, c);
    grid.insert(from, '.');
    c
}

fn move_grid(grid: &mut HashMap<(i32, i32), char>, pos: (i32, i32), m: &Move) -> (i32, i32) {
    let dir = match m.direction {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    };
    let mut scan = pos.clone();
    loop {
        scan = (scan.0 + dir.0, scan.1 + dir.1);
        if !grid.contains_key(&scan) {
            return pos;
        }
        if grid[&scan] == '#' {
            return pos;
        }
        if grid[&scan] == 'O' {
            continue;
        }
        if grid[&scan] == '.' {
            loop {
                let scan2 = (scan.0 - dir.0, scan.1 - dir.1);
                let moved_char = move_char(grid, scan, scan2);
                scan = scan2;
                if moved_char == '@' {
                    return (pos.0 + dir.0, pos.1 + dir.1);
                }
            }
        }
    }
}

struct BigBlockMove {
    blocks: HashMap<(i32, i32), char>,
}

fn move_grid_2(grid: &mut HashMap<(i32, i32), char>, pos: (i32, i32), m: &Move) -> (i32, i32) {
    if m.direction == '<' || m.direction == '>' {
        return move_grid(grid, pos, m);
    }
    let dir = match m.direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    };
    let mut scan_set = HashSet::from([pos.clone()]);
    let mut blocks_to_move = BigBlockMove {
        blocks: HashMap::new(),
    };
    while !scan_set.is_empty() {
        let scan = scan_set.iter().next().unwrap().clone();
        scan_set.remove(&scan);
        let next_pos = (scan.0 + dir.0, scan.1 + dir.1);
        let next_c = grid[&next_pos];
        if next_c == '#' {
            return pos;
        } else if next_c == '.' {
            continue;
        } else if next_c == '[' {
            let next_pos_2 = (next_pos.0, next_pos.1 + 1);
            scan_set.insert(next_pos);
            scan_set.insert(next_pos_2);
            blocks_to_move.blocks.insert(next_pos, '[');
            blocks_to_move.blocks.insert(next_pos_2, ']');
        } else if next_c == ']' {
            let next_pos_2 = (next_pos.0, next_pos.1 - 1);
            scan_set.insert(next_pos);
            scan_set.insert(next_pos_2);
            blocks_to_move.blocks.insert(next_pos, ']');
            blocks_to_move.blocks.insert(next_pos_2, '[');
        }
    }
    blocks_to_move.blocks.iter().for_each(|(p, _)| {
        grid.insert(*p, '.');
    });
    blocks_to_move.blocks.iter().for_each(|(p, c)| {
        grid.insert((p.0 + dir.0, p.1 + dir.1), *c);
    });
    move_grid(grid, pos, m)
}

fn gps_score(grid: &HashMap<(i32, i32), char>, c: char) -> i32 {
    grid.iter()
        .filter(|(_, &c2)| c2 == c)
        .map(|(pos, _)| pos)
        .map(|(x, y)| 100 * x + y)
        .sum()
}

#[allow(unused)]
fn part1(grid: &HashMap<(i32, i32), char>, moves: &Vec<Move>) -> i32 {
    let mut grid = grid.clone();
    let mut pos = find_robot_pos(&grid);
    // pos = move_grid(&mut grid, pos, &Move { direction: '>' });
    moves.iter().for_each(|m| {
        // grid::print_grid(&grid);
        // println!("{:?}", pos);
        // println!("{:?}", m);
        pos = move_grid(&mut grid, pos, m);
    });
    // grid::print_grid(&grid);
    gps_score(&grid, 'O')
}

fn expand_grid(grid: &HashMap<(i32, i32), char>) -> HashMap<(i32, i32), char> {
    let mut new_grid: HashMap<(i32, i32), char> = HashMap::new();
    for ((i, j), &c) in grid.iter() {
        match c {
            '@' => {
                new_grid.insert((*i, 2 * *j), c);
                new_grid.insert((*i, 2 * *j + 1), '.');
            }
            'O' => {
                new_grid.insert((*i, 2 * *j), '[');
                new_grid.insert((*i, 2 * *j + 1), ']');
            }
            _ => {
                new_grid.insert((*i, 2 * *j), c);
                new_grid.insert((*i, 2 * *j + 1), c);
            }
        }
    }
    new_grid
}

#[allow(unused)]
fn part2(grid: &HashMap<(i32, i32), char>, moves: &Vec<Move>) -> i32 {
    let mut grid = expand_grid(grid);
    let mut pos = find_robot_pos(&grid);
    moves.iter().for_each(|m| {
        pos = move_grid_2(&mut grid, pos, m);
    });
    grid::print_grid(&grid);
    // println!("{:?}", pos);
    gps_score(&grid, '[')
}
