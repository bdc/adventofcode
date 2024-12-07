use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

static FNS: [fn(Vec<String>); 4] = [day1::main, day2::main, day3::main, day4::main];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    FNS[day - 1](args.clone().into_iter().skip(2).collect());
}
