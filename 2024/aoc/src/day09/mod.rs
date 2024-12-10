use std::fs;

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

#[derive(Clone)]
enum Item {
    File { id: usize, length: usize },
    Gap { length: usize },
}

fn parse_input(input: &str) -> Vec<Item> {
    let mut items = Vec::new();
    for (id, c) in input.chars().step_by(2).enumerate() {
        items.push(Item::File {
            id,
            length: c.to_digit(10).unwrap() as usize,
        });
        items.push(Item::Gap {
            length: input
                .chars()
                .nth(id * 2 + 1)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap() as usize,
        });
    }
    items.pop();
    items
}

fn score_files(items: Vec<Item>) -> u64 {
    let mut score = 0;
    let mut index = 0;
    for item in items.iter() {
        match item {
            Item::File { id, length } => {
                score += subscore(*id, index, *length);
                index += *length;
            }
            Item::Gap { length } => {
                index += *length;
            }
        }
    }
    score
}

fn subscore(id: usize, index: usize, length: usize) -> u64 {
    if length == 0 {
        return 0;
    }
    let score = (id * (index * length + length * (length - 1) / 2)) as u64;
    score
}

fn calc_1(mut items: Vec<Item>) -> u64 {
    let mut index = 0;
    let mut score: u64 = 0;
    while items.len() > 0 {
        match items[0] {
            Item::File { id, length } => {
                score += subscore(id, index, length);
                index += length;
                items.remove(0);
            }
            Item::Gap { length } => {
                let Item::File {
                    id: l_id_ref,
                    length: l_length_ref,
                } = items.last().unwrap()
                else {
                    panic!("Last item is not a file");
                };

                let l_id = *l_id_ref;
                let l_length = *l_length_ref;

                if l_length < length {
                    score += subscore(l_id, index, l_length);
                    index += l_length;
                    items[0] = Item::Gap {
                        length: length - l_length,
                    };
                    items.pop();
                    items.pop();
                } else if l_length == length {
                    score += subscore(l_id, index, l_length);
                    index += l_length;
                    items.remove(0);
                    items.pop();
                    if matches!(items.last().unwrap(), Item::Gap { .. }) {
                        items.pop();
                    }
                } else if l_length > length {
                    score += subscore(l_id, index, length);
                    index += length;
                    items.remove(0);
                    items.pop();
                    items.push(Item::File {
                        id: l_id,
                        length: l_length - length,
                    });
                }
            }
        }
    }
    score
}

fn calc_2(mut items: Vec<Item>) -> u64 {
    let max_file_id = items.len() / 2;
    for file_id in (0..max_file_id + 1).rev() {
        // println!("File ID: {}", file_id);
        // print_memory(items.clone());
        let file_index = items
            .iter()
            .position(|item| match item {
                Item::File { id, .. } => *id == file_id,
                _ => false,
            })
            .unwrap();
        let file_length = match items[file_index] {
            Item::File { length, .. } => length,
            _ => panic!("File not found"),
        };
        let gap_index = items.iter().position(|item| match item {
            Item::Gap { length } => *length >= file_length,
            _ => false,
        });
        if gap_index.is_none() || gap_index.unwrap() > file_index {
            continue;
        }
        let gap_index = gap_index.unwrap();
        let gap_length = match items[gap_index] {
            Item::Gap { length } => length,
            _ => panic!("Gap not found"),
        };

        // grab the file
        let file_item = items.remove(file_index);

        // replace with gap
        let new_gap_item = Item::Gap {
            length: file_length,
        };
        items.insert(file_index, new_gap_item);

        // insert the file in the new spot and adjust gaps
        items.remove(gap_index);
        items.insert(gap_index, file_item);
        if gap_length > file_length {
            items.insert(
                gap_index + 1,
                Item::Gap {
                    length: gap_length - file_length,
                },
            );
        }
    }

    // print_memory(items.clone());

    // score the files
    score_files(items)
}

#[allow(unused)]
fn print_memory(items: Vec<Item>) {
    for item in items {
        match item {
            Item::File { id, length } => {
                for _ in 0..length {
                    print!("{}", id);
                }
            }
            Item::Gap { length } => {
                for _ in 0..length {
                    print!(".");
                }
            }
        }
    }
    println!();
}

fn part1(input: &Vec<Item>) -> u64 {
    calc_1(input.iter().cloned().collect())
}

fn part2(input: &Vec<Item>) -> u64 {
    calc_2(input.iter().cloned().collect())
}
