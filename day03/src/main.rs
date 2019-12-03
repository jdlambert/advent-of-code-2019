use std::{
    collections::{HashMap, HashSet},
    fs,
};

use std::convert::From;

#[derive(Debug)]
struct WireSection {
    direction: (i32, i32),
    length: u32,
}

impl From<String> for WireSection {
    fn from(string: String) -> Self {
        let mut chars = string.chars();
        let direction = match chars.next() {
            Some('U') => (1, 0),
            Some('D') => (-1, 0),
            Some('R') => (0, 1),
            Some('L') => (0, -1),
            _ => panic!("Unrecognized char!"),
        };
        let length = chars.as_str().parse().unwrap();
        WireSection { direction, length }
    }
}

#[derive(Debug)]
struct Wire {
    sections: Vec<WireSection>,
}

impl From<String> for Wire {
    fn from(string: String) -> Self {
        let mut sections = vec![];
        for s in string.split(",") {
            sections.push(WireSection::from(s.to_string()));
        }
        Wire { sections }
    }
}

impl Wire {
    fn locations(&self) -> HashMap<(i32, i32), u32> {
        let mut locs = HashMap::new();
        let mut cur = (0, 0);
        let mut steps = 0;
        for section in &self.sections {
            let dir = section.direction;
            for _ in 0..section.length {
                steps += 1;
                cur = (cur.0 + dir.0, cur.1 + dir.1);
                locs.insert(cur, steps);
            }
        }
        locs
    }

    fn intersection(&self, other: &Wire) -> HashSet<(i32, i32)> {
        let mine: HashSet<(i32, i32)> = self.locations().keys().cloned().collect();
        let theirs: HashSet<(i32, i32)> = other.locations().keys().cloned().collect();
        mine.intersection(&theirs).cloned().collect()
    }
}

fn part1(wires: (Wire, Wire)) -> u32 {
    let intersection = wires.0.intersection(&wires.1);
    let min = intersection
        .iter()
        .min_by_key(|p| (p.0.abs() + p.1.abs()))
        .unwrap();
    (min.0.abs() + min.1.abs()) as u32
}

fn part2(wires: (Wire, Wire)) -> u32 {
    let intersection = wires.0.intersection(&wires.1);
    let first = wires.0.locations();
    let second = wires.1.locations();
    let min = intersection
        .iter()
        .min_by_key(|p| first.get(p).unwrap() + second.get(p).unwrap())
        .unwrap();
    (first.get(min).unwrap() + second.get(min).unwrap()) as u32
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut lines = content.lines();
    let data = (
        Wire::from(lines.next().unwrap().to_string()),
        Wire::from(lines.next().unwrap().to_string()),
    );
    println!("Part 1: {:?}", part1(data));
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut lines = content.lines();
    let data = (
        Wire::from(lines.next().unwrap().to_string()),
        Wire::from(lines.next().unwrap().to_string()),
    );
    println!("Part 1: {}", part2(data));
}
