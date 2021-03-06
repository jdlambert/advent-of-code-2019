use std::fs;

type Result<T> = std::result::Result<T, &'static str>;

fn execute(program: &Vec<u32>, noun: u32, verb: u32) -> Result<u32> {
    let mut program = program.clone();
    program[1] = noun;
    program[2] = verb;
    for i in (0..program.len()).step_by(4) {
        let op = program[i];
        let a = program[program[i + 1] as usize];
        let b = program[program[i + 2] as usize];
        let out = program[i + 3] as usize;
        match op {
            1 => program[out] = a + b,
            2 => program[out] = a * b,
            99 => return Ok(program[0]),
            _ => return Err("Unexpected opcode!"),
        }
    }
    Err("Never halted!")
}

fn part1(data: &Vec<u32>) -> Result<u32> {
    Ok(execute(data, 12, 2)?)
}

const TARGET: u32 = 19690720;

fn part2(data: &Vec<u32>) -> Result<u32> {
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(data, noun, verb)? == TARGET {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err("No answer in provided range!")
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data = content
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", part1(&data).unwrap());
    println!("Part 1: {}", part2(&data).unwrap());
}
