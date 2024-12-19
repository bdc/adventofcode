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

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut towels: Vec<String> = vec![];
    let mut sequences: Vec<String> = vec![];
    for line in input.lines() {
        if line.contains(",") {
            towels = line.split(", ").map(|s| s.to_string()).collect();
        } else if line.len() > 0 {
            sequences.push(line.to_string());
        }
    }
    towels.sort_by_key(|t| t.len());
    towels.reverse();
    sequences.sort_by_key(|s| s.len());
    sequences.reverse();
    (towels, sequences)
}

struct Node {
    _id: i32,
    b: HashSet<i32>,
    g: HashSet<i32>,
    r: HashSet<i32>,
    u: HashSet<i32>,
    w: HashSet<i32>,
}

impl Node {
    fn new(id: i32) -> Self {
        Node {
            _id: id,
            b: HashSet::new(),
            g: HashSet::new(),
            r: HashSet::new(),
            u: HashSet::new(),
            w: HashSet::new(),
        }
    }
}

struct Map {
    next_id: i32,
    nodes: HashMap<i32, Node>,
}

impl Map {
    fn new() -> Self {
        let id = Self::start_id();
        let start = Node::new(id);
        Map {
            next_id: id + 1,
            nodes: HashMap::from([(id, start)]),
        }
    }

    fn start_id() -> i32 {
        0
    }

    fn add_towel(&mut self, towel: &str) {
        let mut node_id = Self::start_id();
        for (i, c) in towel.chars().enumerate() {
            node_id = self.add_edge(node_id, c, i == towel.len() - 1);
        }
    }

    fn get_edge(&self, node_id: i32, c: char) -> &HashSet<i32> {
        match c {
            'b' => &self.nodes.get(&node_id).unwrap().b,
            'g' => &self.nodes.get(&node_id).unwrap().g,
            'r' => &self.nodes.get(&node_id).unwrap().r,
            'u' => &self.nodes.get(&node_id).unwrap().u,
            'w' => &self.nodes.get(&node_id).unwrap().w,
            _ => panic!("Invalid character"),
        }
    }

    fn get_edge_mut(&mut self, node_id: i32, c: char) -> &mut HashSet<i32> {
        match c {
            'b' => &mut self.nodes.get_mut(&node_id).unwrap().b,
            'g' => &mut self.nodes.get_mut(&node_id).unwrap().g,
            'r' => &mut self.nodes.get_mut(&node_id).unwrap().r,
            'u' => &mut self.nodes.get_mut(&node_id).unwrap().u,
            'w' => &mut self.nodes.get_mut(&node_id).unwrap().w,
            _ => panic!("Invalid character"),
        }
    }

    fn add_edge(&mut self, node_id: i32, c: char, is_terminal: bool) -> i32 {
        let start_id = Self::start_id();
        let edge_set = self.get_edge_mut(node_id, c);
        if is_terminal {
            edge_set.insert(start_id);
            return start_id;
        }
        self.add_node(node_id, c)
    }

    fn add_node(&mut self, source_node_id: i32, c: char) -> i32 {
        let new_node_id = self.next_id;
        self.next_id += 1;
        let new_node = Node::new(new_node_id);
        self.nodes.insert(new_node_id, new_node);
        self.get_edge_mut(source_node_id, c).insert(new_node_id);
        new_node_id
    }

    fn check_sequence(&self, seq: &str) -> bool {
        let mut node_ids: HashSet<i32> = HashSet::from([Self::start_id()]);
        for c in seq.chars() {
            node_ids = node_ids
                .iter()
                .map(|&node_id| self.get_edge(node_id, c))
                .fold(HashSet::new(), |mut acc, edge_set| {
                    acc.extend(edge_set.iter());
                    acc
                });
        }
        node_ids.contains(&Self::start_id())
    }

    fn check_sequence_with_count(&self, seq: &str) -> i64 {
        let mut node_ids: HashMap<i32, i64> = HashMap::from([(Self::start_id(), 1)]);
        for c in seq.chars() {
            let mut next_node_ids = HashMap::new();
            node_ids.iter().for_each(|(&node_id, count)| {
                let edge_set = self.get_edge(node_id, c);
                edge_set.iter().for_each(|&edge_node_id| {
                    *next_node_ids.entry(edge_node_id).or_insert(0) += *count;
                });
            });
            node_ids = next_node_ids;
        }
        *node_ids.get(&Self::start_id()).unwrap_or(&0)
    }

    fn build_map(towels: &Vec<String>) -> Map {
        let mut map = Map::new();
        for towel in towels {
            map.add_towel(towel);
        }
        map
    }
}

#[allow(unused_variables)]
fn part1((towels, sequences): &(Vec<String>, Vec<String>)) -> i32 {
    let map = Map::build_map(towels);
    sequences.iter().filter(|s| map.check_sequence(s)).count() as i32
}

#[allow(unused_variables)]
fn part2((towels, sequences): &(Vec<String>, Vec<String>)) -> i64 {
    let map = Map::build_map(towels);
    sequences
        .iter()
        .map(|s| map.check_sequence_with_count(s))
        .sum()
}
