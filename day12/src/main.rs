use std::fs;

extern crate regex;
use regex::Regex;

extern crate num_integer;
use num_integer::lcm;

#[derive(Debug, Clone)]
struct Moon {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Moon {
    fn energy(&self) -> i64 {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.vel;
        (x.abs() + y.abs() + z.abs()) * (dx.abs() + dy.abs() + dz.abs())
    }
}

fn cmp(a: i64, b: i64) -> i64 {
    if a > b {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

fn part1(mut moons: Vec<Moon>) -> i64 {
    let mut new_moons = vec![];

    for _ in 0..1000 {
        for (i, moon) in moons.iter().enumerate() {
            let mut ddx = 0;
            let mut ddy = 0;
            let mut ddz = 0;
            for (j, other) in moons.iter().enumerate() {
                if i != j {
                    let (ax, ay, az) = moon.pos;
                    let (bx, by, bz) = other.pos;
                    ddx += cmp(ax, bx);
                    ddy += cmp(ay, by);
                    ddz += cmp(az, bz);
                }
            }
            let (dx, dy, dz) = moon.vel;
            let (dx, dy, dz) = (dx + ddx, dy + ddy, dz + ddz);
            let (x, y, z) = moon.pos;
            let (x, y, z) = (x + dx, y + dy, z + dz);

            new_moons.push(Moon {
                pos: (x, y, z),
                vel: (dx, dy, dz),
            })
        }
        moons = std::mem::replace(&mut new_moons, vec![]);
    }

    moons.iter().map(Moon::energy).sum()
}

fn cycle(mut pairs: Vec<(i64, i64)>) -> i64 {
    let target = pairs.clone();
    let mut new_pairs = vec![];
    let mut count = 0;
    loop {
        for (i, moon) in pairs.iter().enumerate() {
            let mut ddx = 0;
            for (j, other) in pairs.iter().enumerate() {
                if i != j {
                    ddx += cmp(moon.0, other.0);
                }
            }
            let dx = moon.1 + ddx;
            let x = moon.0 + dx;

            new_pairs.push((x, dx))
        }
        count += 1;
        pairs = std::mem::replace(&mut new_pairs, vec![]);
        if pairs == target {
            println!("{}", count);
            return count;
        }
    }
}

fn part2(moons: Vec<Moon>) -> i64 {
    let xs = moons.iter().map(|moon| (moon.pos.0, moon.vel.0)).collect();
    let ys = moons.iter().map(|moon| (moon.pos.1, moon.vel.1)).collect();
    let zs = moons.iter().map(|moon| (moon.pos.2, moon.vel.2)).collect();
    let xs = cycle(xs);
    let ys = cycle(ys);
    let zs = cycle(zs);
    return lcm(xs, lcm(ys, zs));
}

fn main() {
    let moon_re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let content = fs::read_to_string("./input.txt").unwrap();
    let moons: Vec<Moon> = content
        .lines()
        .map(|line| {
            let caps = moon_re.captures(&line).unwrap();
            let moon = Moon {
                pos: (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                ),
                vel: (0, 0, 0),
            };
            return moon;
        })
        .collect();

    println!("Part 1: {}", part1(moons.clone()));
    println!("Part 2: {}", part2(moons.clone()));
}
