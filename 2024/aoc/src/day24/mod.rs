use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use regex::Regex;

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

#[derive(Eq, Hash, PartialEq, Clone)]
struct Wire {
    name: String,
    value: usize,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Gate {
    wire1: String,
    wire2: String,
    dest: String,
    op: String,
}

fn parse_input(input: &str) -> (HashMap<String, Wire>, HashSet<Gate>) {
    let mut wires: HashMap<String, Wire> = HashMap::new();
    let mut gates: HashSet<Gate> = HashSet::new();
    let wire_regex = Regex::new(r"^(\w+): (\d)$").unwrap();
    let gate_regex = Regex::new(r"^(\w+) (AND|OR|XOR) (\w+) -> (\w+)$").unwrap();
    input.lines().for_each(|line| {
        if let Some(wire_match) = wire_regex.captures(line) {
            let name = wire_match.get(1).unwrap().as_str().to_string();
            let value = wire_match
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            wires.insert(name.clone(), Wire { name, value });
        } else if let Some(gate_match) = gate_regex.captures(line) {
            let wire1 = gate_match.get(1).unwrap().as_str().to_string();
            let op = gate_match.get(2).unwrap().as_str().to_string();
            let wire2 = gate_match.get(3).unwrap().as_str().to_string();
            let dest = gate_match.get(4).unwrap().as_str().to_string();
            gates.insert(Gate {
                wire1,
                wire2,
                dest,
                op,
            });
        }
    });
    (wires, gates)
}

fn calculate_gate(gate: &Gate, wires: &mut HashMap<String, Wire>) {
    let a = wires.get(&gate.wire1).unwrap();
    let b = wires.get(&gate.wire2).unwrap();
    let c_val = match gate.op.as_str() {
        "AND" => a.value & b.value,
        "OR" => a.value | b.value,
        "XOR" => a.value ^ b.value,
        _ => unreachable!(),
    };
    wires.insert(
        gate.dest.clone(),
        Wire {
            name: gate.dest.clone(),
            value: c_val,
        },
    );
}

fn from_binary(wires: Vec<usize>) -> usize {
    wires.iter().fold(0, |acc, &val| acc * 2 + val)
}

fn get_z_wires(wires: &HashMap<String, Wire>, gates: &HashSet<Gate>) -> Vec<usize> {
    let mut wires = wires.clone();
    let mut num_gates = gates.len();
    while num_gates > 0 {
        let gate = gates
            .iter()
            .find(|gate| {
                wires.contains_key(&gate.wire1)
                    && wires.contains_key(&gate.wire2)
                    && !wires.contains_key(&gate.dest)
            })
            .unwrap();
        calculate_gate(gate, &mut wires);
        num_gates -= 1;
    }
    wires
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted_by_key(|&k| k)
        .rev()
        .map(|key| wires.get(key).unwrap().value)
        .collect_vec()
}

#[allow(unused)]
fn part1(input: &(HashMap<String, Wire>, HashSet<Gate>)) -> usize {
    let wires = &input.0;
    let gates = &input.1;
    let z_wires = get_z_wires(wires, gates);
    from_binary(z_wires)
}

#[allow(unused)]
fn part2(input: &(HashMap<String, Wire>, HashSet<Gate>)) -> String {
    // swaps (found by printing out the graphviz visualization):
    //   1a. y07 AND x07 -/> z07
    //   1b. pmc XOR mvw -/> gmt
    //   2a. x11 AND y11 -/> cbj
    //   2b. y11 XOR x11 -/> qjj
    //   3a. hch XOR nff -/> dmn
    //   3b. khk OR  stg -/> z18
    //   4a. qnm AND rfk -/> z35
    //   4b. qnm XOR rfk -/> cfk
    let swaps = vec![
        "y07 AND x07 -/> z07 -> gmt",
        "pmc XOR mvw -/> gmt -> z07",
        "x11 AND y11 -/> cbj -> qjj",
        "y11 XOR x11 -/> qjj -> cbj",
        "hch XOR nff -/> dmn -> z18",
        "khk OR  stg -/> z18 -> dmn",
        "qnm AND rfk -/> z35 -> cfk",
        "qnm XOR rfk -/> cfk -> z35",
    ];
    swaps
        .iter()
        .map(|swap| swap[23..].to_string())
        .sorted()
        .join(",")
}
