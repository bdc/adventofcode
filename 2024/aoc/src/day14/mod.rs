use regex::Regex;
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

#[derive(Debug, Clone)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn parse_input_to_robot(input: String) -> Robot {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let captures = re.captures(&input).unwrap();
    Robot {
        p: (
            captures[1].parse::<i64>().unwrap(),
            captures[2].parse::<i64>().unwrap(),
        ),
        v: (
            captures[3].parse::<i64>().unwrap(),
            captures[4].parse::<i64>().unwrap(),
        ),
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in input.lines() {
        let robot = parse_input_to_robot(line.to_string());
        robots.push(robot);
    }
    robots
}

fn get_room_size(robots: &Vec<Robot>) -> (i64, i64) {
    if robots.len() == 12 {
        return (11, 7);
    }
    (101, 103)
}

fn move_robot(robot: &mut Robot, room_size: (i64, i64)) -> () {
    let new_x = (robot.p.0 + robot.v.0 + room_size.0) % room_size.0;
    let new_y = (robot.p.1 + robot.v.1 + room_size.1) % room_size.1;
    robot.p = (new_x, new_y);
}

fn move_n_robot(robot: &mut Robot, n: i64, room_size: (i64, i64)) -> () {
    for _ in 0..n {
        move_robot(robot, room_size);
    }
}

fn calc_quadrant(robot: &Robot, room_size: (i64, i64)) -> char {
    // 'A', 'B', 'C', 'D'
    let w = robot.p.0 < (room_size.0 - 1) / 2;
    let e = robot.p.0 > (room_size.0 - 1) / 2;
    let n = robot.p.1 < (room_size.1 - 1) / 2;
    let s = robot.p.1 > (room_size.1 - 1) / 2;
    match (w, e, n, s) {
        (true, false, true, false) => 'A',
        (true, false, false, true) => 'B',
        (false, true, true, false) => 'C',
        (false, true, false, true) => 'D',
        _ => 'Z',
    }
}

fn calc_quadrant_map(robots: &Vec<Robot>, room_size: (i64, i64)) -> HashMap<char, Vec<Robot>> {
    let mut map = HashMap::new();
    for robot in robots {
        let quadrant = calc_quadrant(robot, room_size);
        if quadrant != 'Z' {
            map.entry(quadrant).or_insert(vec![]).push(robot.clone());
        }
    }
    map
}

fn print_robots(robots: &Vec<Robot>) {
    let room_size = get_room_size(robots);
    let mut map = HashMap::new();
    robots.iter().map(|r| r.p).for_each(|p| {
        *map.entry(p).or_insert(0) += 1;
    });
    for y in 0..room_size.1 {
        for x in 0..room_size.0 {
            if map.contains_key(&(x, y)) {
                print!("{}", map[&(x, y)]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_coherence(robots: &Vec<Robot>) -> f32 {
    let mut score = 0.0;
    for robot in robots {
        for other in robots {
            if (robot.p.0 - other.p.0).abs() <= 2 && (robot.p.1 - other.p.1).abs() <= 2 {
                score += 1.0;
            }
        }
    }
    score / (robots.len() as f32)
}

#[allow(unused)]
fn part1(input: &Vec<Robot>) -> i64 {
    let mut robots = input.iter().map(|r| r.clone()).collect::<Vec<Robot>>();
    let room_size = get_room_size(input);
    for robot in robots.iter_mut() {
        move_n_robot(robot, 100, room_size);
    }
    let qm = calc_quadrant_map(&robots, room_size);
    qm.values().map(|v| v.len()).product::<usize>() as i64
}

#[allow(unused)]
fn part2(input: &Vec<Robot>) -> i64 {
    let mut n = 0;
    let mut robots = input.iter().map(|r| r.clone()).collect::<Vec<Robot>>();
    let room_size = get_room_size(input);
    for i in 0..(101 * 103) {
        robots.iter_mut().for_each(|r| move_robot(r, room_size));
        let coherence = count_coherence(&robots);
        if coherence > 5.0 {
            // println!("  -- {} {}", i, coherence);
            n = i + 1;
            break;
        }
    }
    print_robots(&robots);
    n
}
