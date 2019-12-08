use permute;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    fs,
    thread::{spawn, JoinHandle},
};

fn execute(mut program: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> JoinHandle<i32> {
    let handle = spawn(move || {
        let mut i = 0;
        let mut rv = 0;
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
                    program[index] = input.recv().unwrap();
                    i += 2;
                }
                4 => {
                    output.send(a).unwrap_or(());
                    rv = a;
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
                9 => {
                    return rv;
                }
                _ => panic!("Unexpected opcode!"),
            }
        }
    });
    return handle;
}

fn signal_output<'a, T: Iterator<Item = &'a i32>>(program: Vec<i32>, mut sequence: T) -> i32 {
    let (e_out, a_in) = channel();
    let (a_out, b_in) = channel();
    let (b_out, c_in) = channel();
    let (c_out, d_in) = channel();
    let (d_out, e_in) = channel();
    e_out.send(*sequence.next().unwrap()).unwrap();
    a_out.send(*sequence.next().unwrap()).unwrap();
    b_out.send(*sequence.next().unwrap()).unwrap();
    c_out.send(*sequence.next().unwrap()).unwrap();
    d_out.send(*sequence.next().unwrap()).unwrap();
    e_out.send(0).unwrap();
    execute(program.clone(), a_in, a_out);
    execute(program.clone(), b_in, b_out);
    execute(program.clone(), c_in, c_out);
    execute(program.clone(), d_in, d_out);
    let last_handle = execute(program.clone(), e_in, e_out);

    last_handle.join().unwrap()
}

fn part1(program: Vec<i32>) -> i32 {
    permute::permutations_of(&[0i32, 1, 2, 3, 4])
        .map(|combo| signal_output(program.clone(), combo))
        .max()
        .unwrap_or(0)
}

fn part2(program: Vec<i32>) -> i32 {
    permute::permutations_of(&[5i32, 6, 7, 8, 9])
        .map(|combo| signal_output(program.clone(), combo))
        .max()
        .unwrap_or(0)
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data: Vec<i32> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", part1(data.clone()));
    println!("Part 2: {}", part2(data.clone()));
}
