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

fn identify_region(grid: &mut HashMap<(i32, i32), char>) -> HashSet<(i32, i32)> {
    let ((i, j), c) = grid.iter().next().unwrap().clone();
    let mut region: HashSet<(i32, i32)> = HashSet::from([(*i, *j)]);
    let mut search: HashSet<(i32, i32)> = HashSet::from([(*i, *j)]);
    while !search.is_empty() {
        let (i, j) = *search.iter().next().unwrap();
        search.remove(&(i, j));
        let mut to_add: HashSet<(i32, i32)> = HashSet::new();
        [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
            .iter()
            .filter(|(i2, j2)| !region.contains(&(*i2, *j2)))
            .filter(|(i2, j2)| grid.contains_key(&(*i2, *j2)))
            .filter(|(i2, j2)| grid.get(&(*i2, *j2)).unwrap() == c)
            .for_each(|(i2, j2)| {
                to_add.insert((*i2, *j2));
            });
        region.extend(to_add.clone());
        search.extend(to_add.clone());
    }
    region
}

fn get_region_area(region: &HashSet<(i32, i32)>) -> usize {
    region.len()
}

fn get_region_perimeter(region: &HashSet<(i32, i32)>) -> usize {
    region
        .iter()
        .map(|(i, j)| (*i, *j))
        .map(|(i, j)| {
            [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)]
                .iter()
                .filter(|(i2, j2)| !region.contains(&(*i2, *j2)))
                .count()
        })
        .sum()
}

fn get_corners(region: &HashSet<(i32, i32)>, (i, j): (i32, i32)) -> usize {
    let (n, s, w, e, nw, ne, sw, se) = (
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    );
    let triplets = [(n, ne, e), (e, se, s), (s, sw, w), (w, nw, n)];
    triplets
        .iter()
        .map(|(a, b, c)| (region.contains(a), region.contains(b), region.contains(c)))
        .map(|(a, b, c)| match (a, b, c) {
            (true, false, true) => true,
            (false, _, false) => true,
            _ => false,
        })
        .filter(|x| *x)
        .count()
}

fn get_region_sides(region: &HashSet<(i32, i32)>) -> usize {
    region.iter().map(|i| get_corners(region, *i)).sum()
}

fn get_region_price(region: &HashSet<(i32, i32)>) -> usize {
    get_region_area(region) * get_region_perimeter(region)
}

fn get_region_discounted_price(region: &HashSet<(i32, i32)>) -> usize {
    get_region_area(region) * get_region_sides(region)
}

fn part1(input: &HashMap<(i32, i32), char>) -> i32 {
    // grid::print_grid(input);
    let mut grid = input.clone();
    let mut price: i32 = 0;
    while grid.len() >= 1 {
        let region = identify_region(&mut grid);
        for (i, j) in region.iter() {
            grid.remove(&(*i, *j));
        }
        price += get_region_price(&region) as i32;
    }
    price
}

fn part2(input: &HashMap<(i32, i32), char>) -> i32 {
    let mut grid = input.clone();
    let mut price: i32 = 0;
    while grid.len() >= 1 {
        let region = identify_region(&mut grid);
        for (i, j) in region.iter() {
            grid.remove(&(*i, *j));
        }
        price += get_region_discounted_price(&region) as i32;
    }
    price
}
