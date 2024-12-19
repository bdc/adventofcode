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

fn parse_input(input: &str) -> Vec<Point> {
    let mut v: Vec<Point> = vec![];
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        v.push(Point {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        });
    }
    v
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn make_grid(input: &Vec<Point>) -> HashMap<(i32, i32), char> {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let min_x = input.iter().map(|p| p.x).min().unwrap();
    let min_y = input.iter().map(|p| p.y).min().unwrap();
    let max_x = input.iter().map(|p| p.x).max().unwrap();
    let max_y = input.iter().map(|p| p.y).max().unwrap();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            grid.insert((x, y), '.');
        }
    }
    grid
}

fn tick(grid: &mut HashMap<(i32, i32), char>, point: &Point) {
    grid.insert((point.x, point.y), '#');
}

fn get_adjacents(point: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ]
}

fn calculate_path(grid: &HashMap<(i32, i32), char>) -> Option<i32> {
    let max_x = grid.keys().map(|p| p.0).max().unwrap();
    let max_y = grid.keys().map(|p| p.1).max().unwrap();
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut to_visit: HashMap<(i32, i32), i32> = HashMap::new();
    to_visit.insert((0, 0), 0);
    while to_visit.len() > 0 {
        let (&(x, y), &c) = to_visit.iter().min_by_key(|(_, &c)| c).unwrap();
        if x == max_x && y == max_y {
            return Some(c);
        }
        to_visit.remove(&(x, y));
        visited.insert((x, y), c);
        let mut to_add: Vec<(i32, i32)> = vec![];
        get_adjacents((x, y))
            .iter()
            .filter(|p| grid.contains_key(p))
            .filter(|p| !visited.contains_key(p))
            .filter(|p| !to_visit.contains_key(p))
            .filter(|p| *grid.get(p).unwrap() != '#')
            .for_each(|p| {
                to_add.push(p.clone());
            });
        for p in to_add {
            to_visit.insert(p, c + 1);
        }
    }
    None
}

#[allow(dead_code, unused_variables)]
fn part1(input: &Vec<Point>) -> i32 {
    let mut grid = make_grid(input);
    let num_ticks = if input.len() > 1024 { 1024 } else { 12 };
    for i in 0..num_ticks {
        tick(&mut grid, &input[i]);
    }
    calculate_path(&grid).unwrap()
}

#[allow(dead_code, unused_variables)]
fn part2(input: &Vec<Point>) -> String {
    let mut grid = make_grid(input);
    let num_ticks = input.len();
    for i in 0..num_ticks {
        tick(&mut grid, &input[i]);
        if calculate_path(&grid) == None {
            return format!("{},{}", input[i].x, input[i].y);
        }
    }
    unreachable!()
}
