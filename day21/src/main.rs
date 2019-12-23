use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use std::{
    collections::HashMap,
    fs,
};

fn get_val(index: i64, mode: i64, rel: i64, program: &HashMap<i64, i64>) -> i64 {
    let immediate = *program.get(&index).unwrap_or(&0);
    match mode {
        0 => *program.get(&immediate).unwrap_or(&0),
        1 => immediate,
        2 => *program.get(&(rel + immediate)).unwrap_or(&0),
        _ => panic!("Invalid mode!"),
    }
}
fn get_addr(index: i64, mode: i64, rel: i64, program: &HashMap<i64, i64>) -> i64 {
    match mode {
        0 => *program.get(&index).unwrap_or(&0),
        2 => rel + *program.get(&index).unwrap_or(&0),
        _ => panic!("Invalid mode!"),
    }
}

fn execute(
    program: HashMap<i64, i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
) -> JoinHandle<i64> {
    let handle = spawn(move || {
        let mut program = program.clone();
        let mut i = 0;
        let mut rel = 0;
        let mut rv = 0;
        loop {
            let op = program.get(&i).unwrap_or(&0);
            if *op == 99 {
                return rv;
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
                    program.insert(index, input.recv().unwrap_or(-1));
                    i += 2;
                }
                4 => {
                    output.send(a).unwrap_or(());
                    rv = a;
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
                    panic!("Unexpected opcode!");
                }
            }
        }
    });
    return handle;
}

fn part2(program: HashMap<i64, i64>) -> i64 {
    let (my_send, bot_recv) = channel();
    let (bot_send, my_recv) = channel();

    execute(program.clone(), bot_recv, bot_send);

    spawn(move || {
        "OR C J\nAND B J\nAND A J\nNOT J J\nAND D J\nOR E T\nOR H T\nAND T J\nRUN\n".bytes().for_each(|byte| my_send.send(byte as i64).unwrap_or(()));
        loop {
            match my_recv.recv() {
                Ok(i) if i > 128 => break i,
                Ok(i) => print!("{}", (i as u8) as char),
                Err(_) => break 0,
            }
        }
    }).join().unwrap()
}

fn part1(program: HashMap<i64, i64>) -> i64 {
    let (my_send, bot_recv) = channel();
    let (bot_send, my_recv) = channel();

    execute(program.clone(), bot_recv, bot_send);

    spawn(move || {
        "OR C J\nAND B J\nAND A J\nNOT J J\nAND D J\nWALK\n".bytes().for_each(|byte| my_send.send(byte as i64).unwrap_or(()));
        loop {
            match my_recv.recv() {
                Ok(i) if i > 128 => break i,
                Ok(i) => print!("{}", (i as u8) as char),
                Err(_) => break 0,
            }
        }
    }).join().unwrap()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let program: HashMap<i64, i64> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, v)| (i as i64, v))
        .collect();

    println!("Part 1: {}", part1(program.clone()));
    println!("Part 2: {}", part2(program.clone()));
}
