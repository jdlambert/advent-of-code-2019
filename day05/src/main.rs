use std::fs;

type Result<T> = std::result::Result<T, &'static str>;

fn execute(program: &Vec<i32>, input: i32) -> Result<i32> {
    let mut program = program.clone();
    let mut prints = vec![];
    let mut i = 0;
    loop {
        let op = program[i];
        let mut a = *program.get(i + 1).unwrap_or(&0);
        if (op / 100) % 10 == 0 {
            a = *program.get(a as usize).unwrap_or(&0);
        };
        let mut b = *program.get(i + 2).unwrap_or(&0);
        if (op / 1000) % 10 == 0 {
            b = *program.get(b as usize).unwrap_or(&0);
        };
        let out = *program.get(i + 3).unwrap_or(&0) as usize;
        match op % 10 {
            1 => {
                program[out] = a + b;
                i += 4;
            }
            2 => {
                program[out] = a * b;
                i += 4;
            }
            3 => {
                let index = program[i + 1] as usize;
                program[index] = input;
                i += 2;
            }
            4 => {
                prints.push(a);
                i += 2;
            }
            5 => {
                if a != 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                if a == 0 {
                    i = b as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                program[out] = if a < b { 1 } else { 0 };
                i += 4;
            }
            8 => {
                program[out] = if a == b { 1 } else { 0 };
                i += 4;
            }
            9 => return Ok(*prints.last().unwrap()),
            _ => {
                println!("{}", op);
                return Err("Unexpected opcode!");
            }
        }
    }
}

fn part1(data: &Vec<i32>) -> Result<i32> {
    Ok(execute(data, 1)?)
}

fn part2(data: &Vec<i32>) -> Result<i32> {
    Ok(execute(data, 5)?)
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", part1(&data).unwrap());
    println!("Part 1: {}", part2(&data).unwrap());
}
