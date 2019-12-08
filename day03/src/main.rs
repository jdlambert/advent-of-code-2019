use std::{collections::HashSet, fs};

type Wire = Vec<(i32, i32)>;

fn build_subwire((x, y): &(i32, i32), subwire_string: String) -> Wire {
    let mut chars = subwire_string.chars();

    let (dx, dy) = match chars.next().unwrap() {
        'U' => (1, 0),
        'D' => (-1, 0),
        'L' => (0, 1),
        'R' => (0, -1),
        _ => panic!("Unknown direction"),
    };
    let len: i32 = chars.as_str().parse().unwrap();

    (1..=len).map(|i| (x + (i * dx), y + (i * dy))).collect()
}

fn build_wire(wire_string: String) -> Wire {
    let mut wire: Wire = vec![(0, 0)];
    for subwire_str in wire_string.split(',') {
        wire.append(&mut build_subwire(
            &wire.last().unwrap(),
            subwire_str.to_string(),
        ))
    }
    wire
}

fn part1(intersection: HashSet<&&(i32, i32)>) -> i32 {
    intersection
        .iter()
        .map(|p| (p.0.abs() + p.1.abs()))
        .filter(|p| 0 != *p)
        .min()
        .unwrap()
}

fn part2(intersection: HashSet<&&(i32, i32)>, first: &Wire, second: &Wire) -> usize {
    intersection
        .iter()
        .map(|&p| {
            first.iter().position(|x| x == *p).unwrap()
                + second.iter().position(|x| x == *p).unwrap()
        })
        .filter(|p| 0 != *p)
        .min()
        .unwrap()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut lines = content.lines();
    let first = build_wire(lines.next().unwrap().to_string());
    let second = build_wire(lines.next().unwrap().to_string());

    let first_set: HashSet<_> = first.iter().clone().collect();
    let second_set: HashSet<_> = second.iter().clone().collect();
    let intersection: HashSet<_> = first_set.intersection(&second_set).collect();

    println!("Part 1: {}", part1(intersection.clone()));
    println!("Part 2: {}", part2(intersection, &first, &second));
}
