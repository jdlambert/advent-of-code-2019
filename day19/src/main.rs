use itertools::Itertools;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use std::{
    collections::{HashMap, HashSet},
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
    let mut beam = HashSet::new();
    for (x, y) in (0..10000).cartesian_product(0..10000) {
        if beam.contains(&(x - 1, y))
            || beam.contains(&(x, y - 1))
            || beam.contains(&(x - 1, y - 1))
            || (x < 10 && y < 10)
        {
            let (my_send, bot_recv) = channel();
            let (bot_send, my_recv) = channel();

            let bot_handle = execute(program.clone(), bot_recv, bot_send);
            my_send.send(x).unwrap();
            my_send.send(y).unwrap();
            let val = my_recv.recv().unwrap(); 
            
            if val == 1 {
                println!("Inserting {:?}", (x, y));
                beam.insert((x, y));
                if beam.contains(&(x - 99, y))
                    && beam.contains(&(x, y - 99))
                    && beam.contains(&(x - 99, y - 99))
                {
                    return 10000 * (x - 99) + y - 99;
                }
            }
        }
    }
    1
}

fn part1(program: HashMap<i64, i64>) -> usize {
    (0..50)
        .cartesian_product(0..50)
        .filter(|&(x, y)| {
            let (my_send, bot_recv) = channel();
            let (bot_send, my_recv) = channel();

            execute(program.clone(), bot_recv, bot_send);
            my_send.send(x).unwrap();
            my_send.send(y).unwrap();
            my_recv.recv().unwrap() == 1
        })
        .count()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut program: HashMap<i64, i64> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, v)| (i as i64, v))
        .collect();

    println!("Part 1: {}", part1(program.clone()));
    println!("Part 2: {}", part2(program.clone()));
}
