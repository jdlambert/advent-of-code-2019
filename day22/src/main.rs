use std::fs;
extern crate regex;
use regex::Regex;

enum Action {
    Deal(isize),
    Cut(isize),
    Stack,
}

fn part1(actions: &Vec<Action>) -> isize {
    let len = 10007;
    actions.iter().fold(2019, |i, action| match action {
        Action::Deal(n) => (i * n) % len,
        Action::Cut(n) if *n >= 0 => (i - n + len) % len,
        Action::Cut(n) if *n < 0 => (i + n.abs()) % len,
        Action::Stack => len - 1 - i,
        _ => unreachable!(),
    })
}

fn part2() -> &'static str {
    "nothing yet"
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
    println!("Part 2: {}", part2());
}
