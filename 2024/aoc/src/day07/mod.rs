use num_bigint::BigUint;
use regex::Regex;
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

struct Operation {
    target: BigUint,
    operands: Vec<BigUint>,
}

enum Operator {
    Add,
    Mul,
    Con,
}

fn parse_input(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"(\d+): ([\d ]+)").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|c| Operation {
            target: c.get(1).unwrap().as_str().parse().unwrap(),
            operands: c
                .get(2)
                .unwrap()
                .as_str()
                .split(" ")
                .map(|i| i.parse().unwrap())
                .collect(),
        })
        .collect()
}

fn is_successful_operation(operation: &Operation, operators: &Vec<Operator>) -> bool {
    let o1 = operation;
    if o1.operands.len() == 1 {
        return o1.operands[0] == o1.target;
    }
    if o1.target < o1.operands[0] {
        return false;
    }
    let mut os = operators.iter().map(|o| apply_operator(o1, o));
    os.any(|o| is_successful_operation(&o, operators))
}

fn apply_operator(operation: &Operation, operator: &Operator) -> Operation {
    let a = operation.operands[0].clone();
    let b = operation.operands[1].clone();
    let ab = match operator {
        Operator::Add => a + b,
        Operator::Mul => a * b,
        Operator::Con => format!("{}{}", a, b).parse().unwrap(),
    };
    Operation {
        target: operation.target.clone(),
        operands: [&[ab][..], &operation.operands[2..]].concat(),
    }
}

fn part1(input: &Vec<Operation>) -> String {
    let operators = vec![Operator::Add, Operator::Mul];
    input
        .iter()
        .filter(|operation| is_successful_operation(operation, &operators))
        .map(|o| o.target.clone())
        .sum::<BigUint>()
        .to_string()
}

fn part2(input: &Vec<Operation>) -> String {
    let operators = vec![Operator::Add, Operator::Mul, Operator::Con];
    input
        .iter()
        .filter(|operation| is_successful_operation(operation, &operators))
        .map(|o| o.target.clone())
        .sum::<BigUint>()
        .to_string()
}
