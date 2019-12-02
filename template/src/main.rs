use std::fs;

type Result<T> = std::result::Result<T, &'static str>;

fn part1(data: &Vec<u32>) -> &str {
    "nothing yet"
}

fn part2(data: &Vec<u32>) -> &str {
    "nothing yet"
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let data = lines.map(|x| x.parse::<u32>().unwrap()).collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));
}
