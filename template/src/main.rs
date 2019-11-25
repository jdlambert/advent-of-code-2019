use std::{fs, str::Lines};

type Data = i32;

fn parse_data(lines: Lines) -> Vec<Data> {
    lines.map(|x| x.parse::<Data>().unwrap()).collect()
}

fn part1(data: &Vec<Data>) -> &str {
    "nothing yet"
}

fn part2(data: &Vec<Data>) -> &str {
    "nothing yet"
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let data = parse_data(lines);
    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));
}
