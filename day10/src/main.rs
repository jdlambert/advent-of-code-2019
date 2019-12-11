extern crate num_integer;

use itertools::Itertools;
use num_integer::gcd;
use std::{collections::HashSet, f64::consts::PI, fs};

fn first_hit(
    asteroids: &HashSet<(i32, i32)>,
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    bounds_check: fn(i32, i32) -> bool,
) -> Option<(i32, i32)> {
    let (mut i, mut j) = (x, y);
    while bounds_check(i, j) {
        i += dx;
        j += dy;
        if asteroids.contains(&(i, j)) {
            return Some((i, j));
        }
    }
    None
}

fn part1(
    asteroids: &HashSet<(i32, i32)>,
    directions: &Vec<(i32, i32)>,
    bounds_check: fn(i32, i32) -> bool,
) -> usize {
    asteroids
        .iter()
        .map(|&asteroid| {
            directions
                .iter()
                .filter_map(|&direction| first_hit(&asteroids, asteroid, direction, bounds_check))
                .count()
        })
        .max()
        .unwrap()
}

fn direction_to_angle((x, y): &(i32, i32)) -> f64 {
    let d = (*y as f64).atan2(*x as f64) + PI / 2.0;
    if d < 0.0 {
        2.0 * PI + d
    } else {
        d
    }
}

fn part2(
    asteroids: &HashSet<(i32, i32)>,
    directions: &Vec<(i32, i32)>,
    bounds_check: fn(i32, i32) -> bool,
) -> i32 {
    let base = asteroids
        .iter()
        .max_by_key(|&asteroid| {
            directions
                .iter()
                .filter_map(|&direction| first_hit(&asteroids, *asteroid, direction, bounds_check))
                .count()
        })
        .unwrap();

    let mut remaining = asteroids.clone();
    let mut count = 0;
    for direction in directions.iter().cycle() {
        if let Some(asteroid) = first_hit(&remaining, *base, *direction, bounds_check) {
            count += 1;
            remaining.remove(&asteroid);
            if count == 200 {
                return asteroid.0 * 100 + asteroid.1;
            }
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let width = content.find('\n').unwrap() as i32;
    let height = content.matches('\n').count() as i32;

    fn bounds_check(i: i32, j: i32) -> bool {
        return i < 36 && i > -1 && j < 36 && j > -1;
    }

    let asteroids: HashSet<_> = content
        .replace('\n', "")
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == '#' {
                Some((i as i32 % width, i as i32 / width))
            } else {
                None
            }
        })
        .collect();

    let mut directions: Vec<_> = (-(width as i32)..width)
        .cartesian_product(-(height as i32)..height)
        .filter(|&(x, y)| gcd(x, y) == 1)
        .collect();

    directions.sort_by(|a, b| {
        let a = direction_to_angle(a);
        let b = direction_to_angle(b);
        a.partial_cmp(&b).unwrap()
    });

    println!("Part 1: {}", part1(&asteroids, &directions, bounds_check));
    println!("Part 2: {}", part2(&asteroids, &directions, bounds_check));
}
