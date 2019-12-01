use std::{fs, str::Lines};

fn parse_data(lines: Lines) -> Vec<u32> {
    lines.map(|x| x.parse::<u32>().unwrap()).collect()
}

fn part1(data: &Vec<u32>) -> &str {
    "nothing yet"
}

fn part2(data: &Vec<u32>) -> &str {
    "nothing yet"
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let data = parse_data(lines);
    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));
}
