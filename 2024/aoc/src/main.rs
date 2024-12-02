use std::env;

mod day1;
mod day2;

static FNS: [fn(Vec<String>); 2] = [day1::main, day2::main];

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<usize>().unwrap();
    FNS[day - 1](args.clone().into_iter().skip(2).collect());
}
