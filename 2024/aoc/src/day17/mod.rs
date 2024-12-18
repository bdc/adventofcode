use rand::prelude::*;
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

fn parse_input(input: &str) -> ProgramState {
    let re_register = Regex::new(r"Register (\w): (\d+)").unwrap();
    let re_program = Regex::new(r"Program: ([\d,]+)").unwrap();
    let mut ps = ProgramState {
        reg: HashMap::new(),
        program: vec![],
        pointer: 0,
    };
    for line in input.lines() {
        if line.starts_with("Register") {
            let captures = re_register.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().chars().next().unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            ps.reg.insert(name, value);
        } else if line.starts_with("Program") {
            let captures = re_program.captures(line).unwrap();
            let program = captures
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            ps.program = program;
        }
    }
    ps
}

#[derive(Debug, Clone)]
struct ProgramState {
    reg: HashMap<char, i64>,
    program: Vec<usize>,
    pointer: usize,
}

impl ProgramState {
    fn rget(&self, name: char) -> i64 {
        *self.reg.get(&name).unwrap()
    }

    fn rset(&mut self, name: char, value: i64) {
        self.reg.insert(name, value);
    }
}

fn get_operand_value(registers: &HashMap<char, i64>, operator: usize, operand: usize) -> i64 {
    let is_literal = [1, 3].contains(&operator);
    if is_literal {
        operand as i64
    } else {
        match operand {
            0 | 1 | 2 | 3 => operand as i64,
            4 => *registers.get(&'A').unwrap(),
            5 => *registers.get(&'B').unwrap(),
            6 => *registers.get(&'C').unwrap(),
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}

fn step(ps: &mut ProgramState) -> (Option<i64>, bool) {
    let mut output = None;
    if ps.pointer >= ps.program.len() {
        return (output, false);
    }
    let operator = ps.program[ps.pointer];
    let operand = get_operand_value(&ps.reg, operator, ps.program[ps.pointer + 1]);
    // println!(
    //     "pointer: {}, operator: {}, operand: {}",
    //     ps.pointer, operator, operand
    // );
    // let operator_str = match operator {
    //     0 => "0        adv",
    //     1 => " 1       bxl",
    //     2 => "  2      bst",
    //     3 => "   3     jnz",
    //     4 => "    4    bxc",
    //     5 => "     5   out",
    //     6 => "      6  bdv",
    //     7 => "       7 cdv",
    //     _ => panic!("Invalid operator: {}", operator),
    // };
    // println!(
    //     "{} {} {:o} {:o} {:o}",
    //     operator_str,
    //     ps.program[ps.pointer + 1],
    //     ps.rget('A'),
    //     ps.rget('B'),
    //     ps.rget('C')
    // );
    match operator {
        0 => {
            // `adv`
            ps.rset('A', ps.rget('A') / 2_i64.pow(operand as u32));
        }
        1 => {
            // `bxl`
            ps.rset('B', ps.rget('B') ^ operand);
        }
        2 => {
            // `bst`
            ps.rset('B', operand % 8);
        }
        3 => {
            // `jnz`
            if ps.rget('A') != 0 {
                ps.pointer = operand as usize;
                return (output, true);
            }
        }
        4 => {
            // `bxc`
            ps.rset('B', (ps.rget('B') ^ ps.rget('C')) % 8);
        }
        5 => {
            // `out`
            output = Some(operand % 8);
        }
        6 => {
            // `bdv`
            ps.rset('B', ps.rget('A') / 2_i64.pow(operand as u32) % 8);
        }
        7 => {
            // `cdv`
            ps.rset('C', ps.rget('A') / 2_i64.pow(operand as u32) % 8);
        }
        _ => panic!("Invalid operator: {}", operator),
    }
    ps.pointer += 2;
    (output, true)
}

fn calculate_outputs(input: &ProgramState) -> Vec<usize> {
    let mut ps = input.clone();
    let mut outputs: Vec<usize> = vec![];
    while let (output, true) = step(&mut ps) {
        if output.is_some() {
            outputs.push(output.unwrap() as usize);
        }
    }
    outputs
}

fn calculate_outputs_for_a(input: &ProgramState, a: i64) -> Vec<usize> {
    let mut ps = input.clone();
    ps.rset('A', a);
    calculate_outputs(&ps)
}

fn calculate_outputs_str(input: &ProgramState) -> String {
    calculate_outputs(input)
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part1(input: &ProgramState) -> String {
    calculate_outputs_str(input)
}

fn vec_eq(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
}

fn pick_rand(v: &Vec<usize>) -> usize {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..v.len());
    v[i]
}

fn probe(input: &ProgramState) -> Option<i64> {
    let mut a = 0;
    input.program.iter().enumerate().rev().for_each(|(i, _)| {
        // find 'a' such that output is the tail end of input.program
        a <<= 3;
        let valid_a_nexts: Vec<_> = (0..=7)
            .filter(|&a_next| {
                let mut ps = input.clone();
                ps.rset('A', a + a_next);
                let outputs = calculate_outputs(&ps);
                vec_eq(&outputs, &input.program[i..])
            })
            .map(|a_next| a_next as usize)
            .collect();
        // println!("len valid_a_nexts: {}", valid_a_nexts.len());
        if valid_a_nexts.len() > 0 {
            a += pick_rand(&valid_a_nexts) as i64;
        }
    });
    let expected = input.program.clone();
    let actual = calculate_outputs_for_a(&input, a);
    if vec_eq(&expected, &actual) {
        Some(a)
    } else {
        None
    }
}

fn part2(input: &ProgramState) -> String {
    // let expected = input
    //     .program
    //     .iter()
    //     .map(|o| o.to_string())
    //     .collect::<Vec<String>>()
    //     .join(",");
    // println!("{} -- expected", expected);

    let mut iter = 0;
    let mut found: HashMap<i64, i64> = HashMap::new();
    loop {
        iter += 1;
        if iter % 1000 == 0 {
            // println!("{} -- {:?}", iter, found);
        }
        let a = probe(input);
        if a.is_some() {
            let a = a.unwrap();
            found.insert(a, found.get(&a).unwrap_or(&0) + 1);
        }
        if found.len() > 0 && found.values().min().unwrap() > &1 {
            break;
        }
    }
    let min_a = *found.keys().min().unwrap();

    // println!("{} -- {:o}", calculate_outputs_str(&input), min_a);

    min_a.to_string()
}
