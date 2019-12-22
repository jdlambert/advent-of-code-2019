use std::fs;
extern crate regex;
use regex::Regex;

enum Action {
    Deal(i128),
    Cut(i128),
    Stack,
}

fn part1(actions: &Vec<Action>) -> i128 {
    let len = 10007;
    actions.iter().fold(2019, |i, action| match action {
        Action::Stack => len - 1 - i,
        Action::Deal(n) => (i * n),
        Action::Cut(n) if *n >= 0 => (i - n + len),
        Action::Cut(n) if *n < 0 => (i + n.abs()),
        _ => unreachable!(),
    } % len)
}

fn mod_exp(base: i128, exponent: i128, modulus: i128) -> i128 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    loop {
        if exponent <= 0 {
            break result;
        }
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
}

fn part2(actions: &Vec<Action>) -> i128 {
    const LEN: i128 = 119_315_717_514_047;
    const TIMES: i128 = 101_741_582_076_661;

    let (offset_diff, increment_mul) =
        actions
            .clone()
            .iter()
            .fold((0, 1), |(offset, increment), action| {
                let (new_offset, new_increment) = match action {
                    Action::Stack => (offset - increment, -increment),
                    Action::Cut(n) => (offset + increment * *n, increment),
                    Action::Deal(n) => (offset, increment * mod_exp(*n, LEN - 2, LEN)),
                };
                (new_offset % LEN, new_increment % LEN)
            });

    let increment = mod_exp(increment_mul, TIMES, LEN);
    let offset = offset_diff * (1 - mod_exp(increment_mul, TIMES, LEN)) % LEN;
    let offset = offset * mod_exp(1 - increment_mul, LEN - 2, LEN) % LEN;

    (offset + increment * 2020) % LEN
}

fn main() {
    let new_stack_re: Regex = Regex::new(r#"deal into new stack"#).unwrap();
    let deal_re: Regex = Regex::new(r#"deal with increment (\d+)"#).unwrap();
    let cut_re: Regex = Regex::new(r#"cut (-?\d+)"#).unwrap();
    let content = fs::read_to_string("./input.txt").unwrap();
    let actions: Vec<_> = content
        .trim()
        .lines()
        .map(|line| {
            if new_stack_re.is_match(line) {
                Action::Stack
            } else if let Some(caps) = deal_re.captures(line) {
                let interval = caps[1].parse().unwrap();
                Action::Deal(interval)
            } else if let Some(caps) = cut_re.captures(line) {
                let cut = caps[1].parse().unwrap();
                Action::Cut(cut)
            } else {
                unreachable!();
            }
        })
        .collect();
    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));
}
