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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod util;

static FNS: [fn(Vec<String>); 23] = [
    day01::main,
    day02::main,
    day03::main,
    day04::main,
    day05::main,
    day06::main,
    day07::main,
    day08::main,
    day09::main,
    day10::main,
    day11::main,
    day12::main,
    day13::main,
    day14::main,
    day15::main,
    day16::main,
    day17::main,
    day18::main,
    day19::main,
    day20::main,
    day21::main,
    day22::main,
    day23::main,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    FNS[day - 1](args.clone().into_iter().skip(2).collect());
}
