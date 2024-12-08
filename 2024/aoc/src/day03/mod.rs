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

fn parse_input(input: &str) -> String {
    input.lines().collect::<Vec<&str>>().join(" ")
}

fn parse_match_as_int(m: Option<regex::Match>) -> i32 {
    m.map(|s| s.as_str().parse::<i32>()).unwrap().unwrap()
}

fn sum_line(re: &Regex, line: &str) -> i32 {
    re.captures_iter(line)
        .map(|m| (parse_match_as_int(m.get(1)), parse_match_as_int(m.get(2))))
        .map(|(a, b)| a * b)
        .sum()
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    sum_line(&re, input)
}

fn get_active_lines(re: &Regex, input: &str) -> Vec<String> {
    let captures = re.captures_iter(input);
    let mut do_index: Option<usize> = None;
    let mut vec: Vec<String> = vec![];
    captures.for_each(|capture| {
        let c = capture.get(1).unwrap();
        match c.as_str() {
            "do" => {
                if do_index.is_none() {
                    do_index = Some(capture.get(1).unwrap().end());
                }
            }
            "don't" => {
                if let Some(i) = do_index {
                    let s = &input[i..capture.get(1).unwrap().start()];
                    vec.push(s.to_string());
                    do_index = None;
                }
            }
            _ => {}
        }
    });
    vec
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(do|don't)\(\)").unwrap();
    let input = "do()".to_string() + input + "don't()";
    let lines = get_active_lines(&re, &input);
    // for line in lines.clone() {
    //     println!("{}", line);
    // }
    let re2 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    lines.iter().map(|line| sum_line(&re2, line)).sum()
}
