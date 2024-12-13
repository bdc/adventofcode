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

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn move_prize(machine: &Machine) -> Machine {
    Machine {
        a: machine.a,
        b: machine.b,
        prize: (
            machine.prize.0 + 10000000000000,
            machine.prize.1 + 10000000000000,
        ),
    }
}

fn parse_input_to_coordinate(input: String) -> (i64, i64) {
    let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
    // println!("{}", input);
    let captures = re.captures(&input).unwrap();
    (
        captures[1].parse::<i64>().unwrap(),
        captures[2].parse::<i64>().unwrap(),
    )
}

fn parse_input_to_machine(input: (String, String, String)) -> Machine {
    Machine {
        a: parse_input_to_coordinate(input.0),
        b: parse_input_to_coordinate(input.1),
        prize: parse_input_to_coordinate(input.2),
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = vec![];
    for (i, _) in input.lines().enumerate().step_by(4) {
        let machine = parse_input_to_machine((
            input.lines().nth(i).unwrap().to_string(),
            input.lines().nth(i + 1).unwrap().to_string(),
            input.lines().nth(i + 2).unwrap().to_string(),
        ));
        machines.push(machine);
    }
    machines
}

fn calculate_cost(machine: &Machine) -> Option<i64> {
    // it costs 3 tokens to push the A button and 1 token to push the B button

    let a = machine.a;
    let b = machine.b;
    let prize = machine.prize;
    let mut pos = (0, 0);

    let mut a_count = 0;
    let mut b_count = 0;

    while pos.0 <= prize.0 && pos.1 <= prize.1 {
        b_count += 1;
        pos.0 += b.0;
        pos.1 += b.1;
    }

    loop {
        // println!("{:?}", pos);
        if pos == prize {
            return Some(3 * a_count + b_count);
        }
        if pos.0 >= prize.0 || pos.1 >= prize.1 {
            b_count -= 1;
            pos.0 -= b.0;
            pos.1 -= b.1;
        } else if pos.0 < prize.0 && pos.1 < prize.1 {
            a_count += 1;
            pos.0 += a.0;
            pos.1 += a.1;
        }
        if b_count == 0 {
            break;
        }
    }
    None
}

#[allow(unused)]
fn part1(input: &Vec<Machine>) -> i64 {
    // for machine in input {
    //     println!("{:?}", machine);
    //     let cost = calculate_cost(machine);
    //     println!("{:?}", cost);
    // }
    input
        .iter()
        .map(|m| calculate_cost(m))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .sum()
}

fn calculate_cost_efficient(machine: &Machine) -> Option<i64> {
    let (ax, ay) = machine.a;
    let (bx, by) = machine.b;
    let (tx, ty) = machine.prize;

    let n_a = (bx * ty - by * tx) as f64 / (ay * bx - ax * by) as f64;
    let n_b = (ax * ty - ay * tx) as f64 / (ax * by - ay * bx) as f64;

    if n_a == (n_a as i64) as f64 && n_b == (n_b as i64) as f64 {
        return Some(3 * n_a as i64 + n_b as i64);
    }
    None
}

#[allow(unused)]
fn part2(input: &Vec<Machine>) -> i64 {
    let machines: Vec<Machine> = input.iter().map(|m| move_prize(m)).collect();
    for machine in machines.iter() {
        if machine.a.0 * machine.b.1 == machine.a.1 * machine.b.0 {
            // Proving there are no machines with colinear buttons; thus a solution is unique
            println!("a and b buttons are aligned -- {:?}", machine);
        }
    }
    input
        .iter()
        .map(|m| move_prize(m))
        .map(|m| calculate_cost_efficient(&m))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .sum()
}
