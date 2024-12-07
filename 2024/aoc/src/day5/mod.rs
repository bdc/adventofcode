use itertools::iproduct;
use regex::Regex;
use std::{collections::HashSet, fs};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    a: u32,
    b: u32,
}

struct PageSet {
    pages: Vec<u32>,
}

fn parse_input(input: &str) -> (HashSet<Rule>, Vec<PageSet>) {
    let rule_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let rules = input
        .lines()
        .map(|line| rule_re.captures(line))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .map(|c| Rule {
            a: c[1].parse().unwrap(),
            b: c[2].parse().unwrap(),
        })
        .collect();
    let pageset_re = Regex::new(r",").unwrap();
    let pagesets = input
        .lines()
        .filter(|line| pageset_re.is_match(line))
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .map(|pages| PageSet { pages })
        .collect();
    (rules, pagesets)
}

fn is_valid_pageset(pageset: &PageSet, rules: &HashSet<Rule>) -> bool {
    let mut page_pairs = iproduct!(
        pageset.pages.iter().enumerate(),
        pageset.pages.iter().enumerate()
    );
    page_pairs.all(|(a, b)| is_valid_page_pair(a, b, rules))
}

fn is_valid_page_pair(a: (usize, &u32), b: (usize, &u32), rules: &HashSet<Rule>) -> bool {
    let result = a.0 >= b.0 || !rules.contains(&Rule { a: *b.1, b: *a.1 });
    result
}

fn get_middle_page(pageset: &PageSet) -> u32 {
    pageset.pages[pageset.pages.len() / 2]
}

fn part1(input: &(HashSet<Rule>, Vec<PageSet>)) -> u32 {
    input
        .1
        .iter()
        .filter(|pageset| is_valid_pageset(pageset, &input.0))
        .map(|pageset| get_middle_page(&pageset))
        .sum()
}

fn sort(pageset: &PageSet, rules: &HashSet<Rule>) -> PageSet {
    let mut oldpages: HashSet<u32> = pageset.pages.iter().map(|p| *p).collect();
    let mut newpages = vec![];
    while oldpages.len() > 0 {
        let mut nextpage = None;
        for page1 in oldpages.iter() {
            if oldpages
                .iter()
                .all(|page2| is_valid_page_pair((1, page1), (2, page2), rules))
            {
                nextpage = Some(*page1);
                break;
            }
        }
        newpages.push(nextpage.unwrap());
        oldpages.remove(&nextpage.unwrap());
    }
    PageSet { pages: newpages }
}

fn part2(input: &(HashSet<Rule>, Vec<PageSet>)) -> u32 {
    input
        .1
        .iter()
        .filter(|pageset| !is_valid_pageset(pageset, &input.0))
        .map(|pageset| sort(pageset, &input.0))
        .map(|pageset| get_middle_page(&pageset))
        .sum()
}
