use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn orbits_to_center<'a>(planet: &'a str, orbits: &'a HashMap<&str, &str>) -> Vec<&'a str> {
    let mut current = planet;
    let mut orbited = vec![];
    while current != "COM" {
        current = orbits.get(current).unwrap();
        orbited.push(current);
    }
    orbited
}

fn part1(orbits: &HashMap<&str, &str>) -> u32 {
    orbits
        .keys()
        .map(|planet| orbits_to_center(planet, orbits).len() as u32)
        .sum()
}

fn part2(orbits: &HashMap<&str, &str>) -> u32 {
    let you_set: HashSet<_> = orbits_to_center("YOU", orbits).into_iter().collect();
    let san_set: HashSet<_> = orbits_to_center("SAN", orbits).into_iter().collect();

    let difference: HashSet<_> = you_set.symmetric_difference(&san_set).collect();

    difference.iter().len() as u32
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let orbits: HashMap<_,_> = content
        .trim()
        .lines()
        .map(|line| line.split(')').collect::<Vec<&str>>())
        .map(|pair| (pair[1], pair[0]))
        .collect();

    println!("Part 1: {}", part1(&orbits));
    println!("Part 1: {}", part2(&orbits));
}
