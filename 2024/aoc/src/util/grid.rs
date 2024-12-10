use std::collections::HashMap;

#[allow(unused)]
pub fn parse_input_to_grid(input: &str) -> HashMap<(i32, i32), char> {
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

#[allow(unused)]
pub fn print_grid(grid: &HashMap<(i32, i32), char>) {
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