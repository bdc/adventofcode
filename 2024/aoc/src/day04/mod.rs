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

    let parsed = parse_input(&input);
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    i: i32,
    j: i32,
}

impl Coord {
    fn translate(&self, dir: &Dir, amount: i32) -> Self {
        let i = self.i;
        let j = self.j;
        match dir {
            Dir::N => Coord { i: i - amount, j },
            Dir::E => Coord { i, j: j + amount },
            Dir::S => Coord { i: i + amount, j },
            Dir::W => Coord { i, j: j - amount },
            Dir::NE => Coord {
                i: i - amount,
                j: j + amount,
            },
            Dir::SE => Coord {
                i: i + amount,
                j: j + amount,
            },
            Dir::SW => Coord {
                i: i + amount,
                j: j - amount,
            },
            Dir::NW => Coord {
                i: i - amount,
                j: j - amount,
            },
        }
    }
}

enum Dir {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

const ALL_DIRS: [Dir; 8] = [
    Dir::N,
    Dir::E,
    Dir::S,
    Dir::W,
    Dir::NE,
    Dir::SE,
    Dir::SW,
    Dir::NW,
];

fn parse_input(input: &str) -> HashMap<Coord, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                (
                    Coord {
                        i: i as i32,
                        j: j as i32,
                    },
                    c,
                )
            })
        })
        .collect()
}

fn check_x(input: &HashMap<Coord, char>, coord: &Coord) -> u32 {
    ALL_DIRS
        .iter()
        .map(|dir| check_x_dir(input, coord, dir))
        .filter(|b| *b)
        .count() as u32
}

fn check_x_dir(input: &HashMap<Coord, char>, coord: &Coord, dir: &Dir) -> bool {
    let letters = ['X', 'M', 'A', 'S'];
    letters
        .iter()
        .enumerate()
        .filter(|(i, c)| {
            let key = coord.clone().translate(dir, *i as i32);
            !(input.get(&key).is_some_and(|val| val == *c))
        })
        .count()
        == 0
}

fn check_a(input: &HashMap<Coord, char>, coord: &Coord) -> bool {
    [Dir::NW, Dir::NE]
        .iter()
        .all(|dir| check_a_dir(input, coord, dir))
}

fn check_a_dir(input: &HashMap<Coord, char>, coord: &Coord, dir: &Dir) -> bool {
    let letter1 = coord.clone().translate(dir, 1);
    let letter2 = coord.clone().translate(dir, -1);
    let s1 = [letter1, letter2]
        .iter()
        .map(|c| input.get(c))
        .collect::<HashSet<Option<&char>>>();
    let s2 = ['M', 'S'].iter().map(|c| Some(c)).collect::<HashSet<_>>();
    s1.eq(&s2)
}

fn part1(input: &HashMap<Coord, char>) -> u32 {
    input
        .iter()
        .filter(|(_, c)| c == &&'X')
        .map(|(coord, _)| check_x(input, coord))
        .sum()
}

fn part2(input: &HashMap<Coord, char>) -> i32 {
    input
        .iter()
        .filter(|(_, c)| c == &&'A')
        .filter(|(coord, _)| check_a(input, coord))
        .count() as i32
}
