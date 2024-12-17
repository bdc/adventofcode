use crate::util::grid;
use std::{
    collections::{BTreeMap, HashMap},
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

struct PriorityQueue<T> {
    queue: BTreeMap<i32, Vec<T>>,
}

impl<T> PriorityQueue<T> {
    fn new() -> Self {
        Self {
            queue: BTreeMap::new(),
        }
    }

    fn push(&mut self, priority: i32, item: T) {
        self.queue.entry(priority).or_default().push(item);
    }

    fn pop(&mut self) -> Option<(i32, T)> {
        let first = self.queue.pop_first();
        if first.is_none() {
            return None;
        }
        let (k, mut v_list) = first.unwrap();
        assert!(!v_list.is_empty());

        let v = v_list.pop().unwrap();
        if !v_list.is_empty() {
            self.queue.insert(k, v_list);
        }
        Some((k, v))
    }
}

fn traverse(
    grid: &HashMap<(i32, i32), char>,
    start: (i32, i32),
    goal: (i32, i32),
    dir: char,
) -> (i32, char) {
    let mut grid = grid.clone();
    let mut queue: PriorityQueue<(i32, i32, char)> = PriorityQueue::new();
    queue.push(0, (start.0, start.1, dir));
    while let Some((cost, (i, j, dir))) = queue.pop() {
        if !grid.contains_key(&(i, j)) {
            continue;
        }
        if (i, j) == goal {
            return (cost, dir);
        }
        if grid[&(i, j)] != '.' && grid[&(i, j)] != 'S' {
            continue;
        }
        grid.insert((i, j), 'O');
        match dir {
            '>' => {
                queue.push(cost + 1, (i, j + 1, dir));
                queue.push(cost + 1001, (i - 1, j, '^'));
                queue.push(cost + 1001, (i + 1, j, 'v'));
            }
            '<' => {
                queue.push(cost + 1, (i, j - 1, dir));
                queue.push(cost + 1001, (i - 1, j, '^'));
                queue.push(cost + 1001, (i + 1, j, 'v'));
            }
            'v' => {
                queue.push(cost + 1, (i + 1, j, dir));
                queue.push(cost + 1001, (i, j - 1, '<'));
                queue.push(cost + 1001, (i, j + 1, '>'));
            }
            '^' => {
                queue.push(cost + 1, (i - 1, j, dir));
                queue.push(cost + 1001, (i, j - 1, '<'));
                queue.push(cost + 1001, (i, j + 1, '>'));
            }
            _ => unreachable!(),
        }
    }
    (i32::MAX / 2, dir)
}

fn get_pos(grid: &HashMap<(i32, i32), char>, c: char) -> (i32, i32) {
    *grid.iter().find(|(_, &v)| v == c).unwrap().0
}

fn part1(input: &HashMap<(i32, i32), char>) -> i32 {
    let start = get_pos(&input, 'S');
    let goal = get_pos(&input, 'E');
    let (cost, _) = traverse(&input, start, goal, '>');
    cost
}

fn part2(input: &HashMap<(i32, i32), char>) -> i32 {
    let part_1_cost = part1(input);
    let start = get_pos(&input, 'S');
    let goal = get_pos(&input, 'E');
    let mut count = 0;
    input
        .iter()
        .filter(|(_, &c)| "S.E".contains(c))
        .for_each(|(k, _)| {
            let (cost1, dir1) = traverse(&input, start, *k, '>');
            let (cost2, _) = traverse(&input, *k, goal, dir1);
            if cost1 + cost2 == part_1_cost {
                count += 1;
            }
        });
    count
}
