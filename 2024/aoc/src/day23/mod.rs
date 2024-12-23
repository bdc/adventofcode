use std::{collections::HashSet, fs};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Connection<'a> {
    m1: &'a str,
    m2: &'a str,
}

struct SortedNlet {
    items: HashSet<String>,
}

impl SortedNlet {
    fn new() -> Self {
        Self {
            items: HashSet::new(),
        }
    }

    fn add(&mut self, key: Vec<&str>) {
        let mut key = key.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        key.sort();
        self.items.insert(key.join(","));
    }

    fn contains(&self, key: Vec<&str>) -> bool {
        let mut key = key.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        key.sort();
        self.items.contains(&key.join(","))
    }
}

fn parse_input(input: &str) -> (HashSet<String>, HashSet<Connection>) {
    let mut machines: HashSet<String> = HashSet::new();
    let mut connections: HashSet<Connection> = HashSet::new();
    input.lines().for_each(|line| {
        let (m1, m2) = line.split_once('-').unwrap();
        machines.insert(m1.to_string());
        machines.insert(m2.to_string());
        let c1 = Connection { m1: m1, m2: m2 };
        let c2 = Connection { m1: m2, m2: m1 };
        connections.insert(c1);
        connections.insert(c2);
    });
    (machines, connections)
}

fn get_triplets(
    machines: &HashSet<String>,
    connections: &HashSet<Connection>,
) -> HashSet<(String, String, String)> {
    let mut triplets: HashSet<(String, String, String)> = HashSet::new();
    for m1 in machines.iter() {
        for m2 in machines.iter() {
            for m3 in machines.iter() {
                let c1 = Connection { m1: m1, m2: m2 };
                let c2 = Connection { m1: m2, m2: m3 };
                let c3 = Connection { m1: m1, m2: m3 };

                if connections.contains(&c1)
                    && connections.contains(&c2)
                    && connections.contains(&c3)
                {
                    let mut ms = vec![m1.clone(), m2.clone(), m3.clone()];
                    ms.sort();
                    triplets.insert((
                        ms.get(0).unwrap().clone(),
                        ms.get(1).unwrap().clone(),
                        ms.get(2).unwrap().clone(),
                    ));
                }
            }
        }
    }
    triplets
}

#[allow(unused)]
fn part1(input: &(HashSet<String>, HashSet<Connection>)) -> usize {
    let (machines, connections) = input;
    let triplets = get_triplets(machines, connections);
    triplets
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

fn get_nplusonelets(machines: &HashSet<String>, nlets: &SortedNlet) -> SortedNlet {
    let mut nplusonelets = SortedNlet::new();
    for nlet in nlets.items.iter() {
        for m in machines.iter() {
            let mut nlet_items = nlet.split(',').collect::<HashSet<_>>();
            if nlet_items.contains(m.as_str()) {
                continue;
            }
            nlet_items.insert(m.as_str());
            if nlet_items.iter().all(|m| {
                let permutation = nlet_items
                    .iter()
                    .filter(|m2| m2 != &m)
                    .map(|&m2| m2)
                    .collect::<Vec<_>>();
                nlets.contains(permutation)
            }) {
                nplusonelets.add(nlet_items.iter().map(|&m| m).collect::<Vec<_>>());
            }
        }
    }
    nplusonelets
}

#[allow(unused)]
fn part2(input: &(HashSet<String>, HashSet<Connection>)) -> String {
    let (machines, connections) = input;
    let mut nlets = SortedNlet::new();
    get_triplets(machines, connections)
        .iter()
        // .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .for_each(|(a, b, c)| nlets.add(vec![a, b, c]));

    let mut items: HashSet<String>;
    loop {
        nlets = get_nplusonelets(machines, &nlets);
        if nlets.items.len() <= 1 {
            items = nlets.items;
            break;
        }
    }
    items.iter().find(|_| true).unwrap().to_string()
}
