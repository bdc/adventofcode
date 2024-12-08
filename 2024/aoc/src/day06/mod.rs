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

    let parsed = parse_input_to_grid(&input);
    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}

fn parse_input_to_grid(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect()
}

fn step(grid: &mut HashMap<(i32, i32), char>) -> Result<(), &'static str> {
    let guard_ijv = find_guard_ijv(grid)?;
    let guard_ij = (guard_ijv.0, guard_ijv.1);
    let forward_guard_ij = get_forward_guard_ij(guard_ijv);
    if !grid.contains_key(&forward_guard_ij) {
        // Guard exits the grid
        grid.insert(guard_ij, 'X');
    } else if grid.get(&forward_guard_ij) == Some(&'#') {
        // Guard encounters obstacle
        let v = rotate_right(guard_ijv.2);
        grid.insert(guard_ij, v);
    } else {
        // Guard moves forward
        grid.insert(forward_guard_ij, guard_ijv.2);
        grid.insert(guard_ij, 'X');
    }
    Ok(())
}

fn rotate_right(v: char) -> char {
    match v {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid guard direction"),
    }
}

fn get_forward_guard_ij(guard_ijv: (i32, i32, char)) -> (i32, i32) {
    return match guard_ijv.2 {
        '^' => (guard_ijv.0 - 1, guard_ijv.1 + 0),
        'v' => (guard_ijv.0 + 1, guard_ijv.1 + 0),
        '<' => (guard_ijv.0 + 0, guard_ijv.1 - 1),
        '>' => (guard_ijv.0 + 0, guard_ijv.1 + 1),
        _ => panic!("Invalid guard direction"),
    };
}

fn find_guard_ijv(grid: &HashMap<(i32, i32), char>) -> Result<(i32, i32, char), &'static str> {
    let guard_chars = vec!['^', 'v', '<', '>'];
    grid.iter()
        .filter(|(_, v)| guard_chars.contains(v))
        .map(|((i, j), v)| (*i, *j, *v))
        .next()
        .ok_or("No guard found")
}

fn _add_obstacle(grid: &mut HashMap<(i32, i32), char>) {
    let guard_ijv_result = find_guard_ijv(grid);
    if guard_ijv_result.is_err() {
        return;
    }
    let guard_ijv = guard_ijv_result.unwrap();
    let forward_guard_ij = get_forward_guard_ij(guard_ijv);
    if grid.contains_key(&forward_guard_ij) {
        grid.insert(forward_guard_ij, '#');
    }
}

fn is_loop(grid: &mut HashMap<(i32, i32), char>) -> bool {
    let guard_ijv_result = find_guard_ijv(grid);
    if guard_ijv_result.is_err() {
        return false;
    }
    let guard_ijv = guard_ijv_result.unwrap();
    let mut visited: HashSet<(i32, i32, char)> = HashSet::from([guard_ijv]);
    while step(grid).is_ok() {
        let guard_ijv_result = find_guard_ijv(grid);
        if guard_ijv_result.is_err() {
            return false;
        }
        let guard_ijv = guard_ijv_result.unwrap();
        if visited.contains(&guard_ijv) {
            return true;
        }
        visited.insert(guard_ijv);
    }
    false
}

fn _print_grid(grid: &HashMap<(i32, i32), char>) {
    let min_i = *grid.keys().map(|(i, _)| i).min().unwrap();
    let max_i = *grid.keys().map(|(i, _)| i).max().unwrap();
    let min_j = *grid.keys().map(|(_, j)| j).min().unwrap();
    let max_j = *grid.keys().map(|(_, j)| j).max().unwrap();
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            print!("{}", grid.get(&(i, j)).unwrap_or(&' '));
        }
        println!();
    }
}

fn populate_xs(grid: &mut HashMap<(i32, i32), char>) {
    while step(grid).is_ok() {}
}

fn part1(input: &HashMap<(i32, i32), char>) -> u32 {
    let mut grid = input.clone();
    populate_xs(&mut grid);
    // _print_grid(&grid);
    grid.values().filter(|v| v == &&'X').count() as u32
}

fn part2(input: &HashMap<(i32, i32), char>) -> u32 {
    let mut grid = input.clone();
    populate_xs(&mut grid);
    let mut loops = 0;
    let mut non_loops = 0;
    grid.keys().filter(|k| grid.get(k) == Some(&'X')).for_each(|k| {
        let mut grid2 = input.clone();
        grid2.insert(*k, '#');
        match is_loop(&mut grid2) {
            true => loops += 1,
            false => non_loops += 1,
        }
        if (loops + non_loops) % 10 == 0 {
            println!("{} {}", loops, non_loops);
        }
    });
    // while step(&mut grid).is_ok() {
    //     let mut grid2 = grid.clone();
    //     add_obstacle(&mut grid2);
    //     if is_loop(&mut grid2) {
    //         loops += 1;
    //     } else {
    //         non_loops += 1;
    //     }
    //     if (loops + non_loops) % 10 == 0 {
    //         println!("{} {}", loops, non_loops);
    //     }
    // }
    loops
}
