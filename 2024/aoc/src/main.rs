use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

static FNS: [fn(Vec<String>); 7] = [
    day1::main,
    day2::main,
    day3::main,
    day4::main,
    day5::main,
    day6::main,
    day7::main,
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    FNS[day - 1](args.clone().into_iter().skip(2).collect());
}
