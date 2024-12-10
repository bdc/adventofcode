use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod util;

static FNS: [fn(Vec<String>); 9] = [
    day01::main,
    day02::main,
    day03::main,
    day04::main,
    day05::main,
    day06::main,
    day07::main,
    day08::main,
    day09::main,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    FNS[day - 1](args.clone().into_iter().skip(2).collect());
}
