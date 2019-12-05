use std::{fs, collections::{HashSet, HashMap}};

extern crate regex;
use regex::Regex;

fn frequencies(code: &str) -> HashSet<u32> {
    let mut characters = HashMap::new();

    for c in code.chars() {
        let entry = characters.entry(c).or_insert(0);
        *entry += 1u32;
    }

    characters.values().cloned().collect()
}

fn is_monotonic(string: String) -> bool {
    let slice: &str = &string[..];

    let mut chars: Vec<char> = slice.chars().collect();
    chars.sort();
    let sorted: String = chars.into_iter().collect();
    sorted == string
}

fn part1(codes: Vec<String>) -> usize {
    codes.iter()
         .filter(|s| frequencies(s).contains(&2))
         .count()
}

fn part2(codes: Vec<String>) -> usize {
    codes.iter()
         .filter(|s| frequencies(s).iter().any(|v| *v > 1))
         .count()
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let content = fs::read_to_string("./input.txt").unwrap();
    let caps = re.captures(&content).unwrap();
    let first: u32 = caps[1].parse().unwrap();
    let last: u32 = caps[2].parse().unwrap();

    let codes: Vec<String> = (first..=last).map(|i| format!("{}", i))
                                           .filter(|s| is_monotonic(s.clone()))
                                           .collect();
    
    println!("Part 1: {}", part1(codes.clone()));
    println!("Part 1: {}", part2(codes.clone()));
}
