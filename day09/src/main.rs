use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, &'static str>;

fn get_val(index: i64, mode: i64, rel: i64, program: &HashMap<i64, i64>) -> i64 {
    let immediate = *program.get(&index).unwrap_or(&0);
    match mode {
        0 => *program.get(&immediate).unwrap_or(&0),
        1 => immediate,
        2 => *program.get(&(rel + immediate)).unwrap_or(&0),
        _ => panic!("Unkown mode!"),
    }
}
fn get_addr(index: i64, mode: i64, rel: i64, program: &HashMap<i64, i64>) -> i64 {
    match mode {
        0 => *program.get(&index).unwrap_or(&0),
        2 => rel + *program.get(&index).unwrap_or(&0),
        _ => panic!("Unkown mode!"),
    }
}

fn execute(program: &HashMap<i64, i64>, input: i64) -> Result<i64> {
    let mut program = program.clone();
    let mut prints = vec![];
    let mut i = 0;
    let mut rel = 0;
    loop {
        let op = program.get(&i).unwrap_or(&0);
        if *op == 99 {
            return Ok(*prints.last().unwrap());
        }
        let a = get_val(i + 1, (op / 100) % 10, rel, &program);
        let b = get_val(i + 2, (op / 1000) % 10, rel, &program);
        let out = get_addr(i + 3, (op / 10000) % 10, rel, &program);
        match op % 10 {
            1 => {
                program.insert(out, a + b);
                i += 4;
            }
            2 => {
                program.insert(out, a * b);
                i += 4;
            }
            3 => {
                let index = get_addr(i + 1, (op / 100) % 10, rel, &program);
                program.insert(index, input);
                i += 2;
            }
            4 => {
                prints.push(a);
                i += 2;
            }
            5 => {
                if a != 0 {
                    i = b;
                } else {
                    i += 3;
                }
            }
            6 => {
                if a == 0 {
                    i = b;
                } else {
                    i += 3;
                }
            }
            7 => {
                program.insert(out, if a < b { 1 } else { 0 });
                i += 4;
            }
            8 => {
                program.insert(out, if a == b { 1 } else { 0 });
                i += 4;
            }
            9 => {
                rel += a;
                i += 2;
            }
            _ => {
                return Err("Unexpected opcode!");
            }
        }
    }
}

fn part1(data: &HashMap<i64, i64>) -> Result<i64> {
    Ok(execute(data, 1)?)
}

fn part2(data: &HashMap<i64, i64>) -> Result<i64> {
    Ok(execute(data, 2)?)
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut data: HashMap<i64, i64> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, v)| (i as i64, v))
        .collect();

    println!("Part 1: {}", part1(&data).unwrap());
    println!("Part 2: {}", part2(&data).unwrap());
}
