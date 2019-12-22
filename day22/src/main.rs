use std::fs;
extern crate regex;
use regex::Regex;

fn action(deck: &Vec<usize>, action_str: &str) -> Vec<usize> {
    let new_stack_re = Regex::new(r#"deal into new stack"#).unwrap();
    let deal_re = Regex::new(r#"deal with increment (\d+)"#).unwrap();
    let cut_re = Regex::new(r#"cut (-?\d+)"#).unwrap();
    if new_stack_re.is_match(action_str) {
        deck.iter().cloned().rev().collect()
    } else if let Some(caps) = deal_re.captures(action_str) {
        let interval: usize = caps[1].parse().unwrap();
        let len = deck.len();
        let mut cloned = deck.clone();
        let mut redeal = vec![0; len];
        let mut index = 0;
        while cloned.len() > 0 {
            redeal[index % len] = cloned.remove(0);
            index += interval;
        }
        redeal
    } else if let Some(caps) = cut_re.captures(action_str) {
        let cut: usize = match caps[1].parse::<isize>().unwrap() {
            i if i >= 0 => i as usize,
            i if i < 0 => deck.len() - i.abs() as usize,
            _ => unreachable!(),
        };
            let top = deck.iter().take(cut as usize);
            let bottom = deck.iter().skip(cut as usize);
            bottom.chain(top).cloned().collect()
    } else {
        unreachable!();
    }
}

fn part1() -> usize {
    let mut deck: Vec<usize> = (0..10007).collect();
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.trim().lines();
    for line in lines {
        deck = action(&deck, line);
        println!("{}", deck.len());
    }
        println!("{:?}", deck);
   deck.iter().position(|v| *v == 2019).unwrap()
}

fn part2() -> &'static str {
    "nothing yet"
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
