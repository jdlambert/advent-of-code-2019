use std::fs;

fn execute(program: &Vec<u32>, noun: u32, verb: u32) -> u32 {
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
            99 => return program[0],
            _ => panic!("Unexpected opcode!"),
        }
    }
    panic!("Never halted!")
}

fn part1(data: &Vec<u32>) -> u32 {
    execute(data, 12, 2)
}

const TARGET: u32 = 19690720;

fn part2(data: &Vec<u32>) -> u32 {
    for noun in 0..100 {
        for verb in 0..100 {
            if execute(data, noun, verb) == TARGET {
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data = content
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));
}
