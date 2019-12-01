use std::{fs, str::Lines};

fn parse_data(lines: Lines) -> Vec<i32> {
    lines.map(|x| x.parse::<i32>().unwrap()).collect()
}

fn fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn part1(data: &Vec<i32>) -> i32 {
    data.iter().map(|x| fuel(*x)).sum()
}

fn total_fuel(mass: i32) -> i32 {
    let fuel = fuel(mass);
    if fuel < 0 {
        0
    } else {
        fuel + total_fuel(fuel)
    }
}

fn part2(data: &Vec<i32>) -> i32 {
    data.iter().map(|x| total_fuel(*x)).sum()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let data = parse_data(lines);
    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));
}
