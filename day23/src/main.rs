use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

use std::{collections::HashMap, fs};

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
                    program.insert(
                        index,
                        input.recv_timeout(Duration::from_millis(3)).unwrap_or(-1),
                    );
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
    let mut my_sends = vec![];
    let mut my_recvs = vec![];
    for i in 0..50 {
        let (my_send, bot_recv) = channel();
        let (bot_send, my_recv) = channel();
        execute(program.clone(), bot_recv, bot_send);
        my_send.send(i).unwrap();
        my_sends.push(my_send);
        my_recvs.push(my_recv);
    }
    spawn(move || {
        let mut last_y = None;
        let mut current_nat = None;
        loop {
            let mut all_idle = true;
            for recv in &my_recvs {
                if let Ok(addr) = recv.recv_timeout(Duration::from_millis(1)) {
                    all_idle = false;
                    let x = recv.recv().unwrap();
                    let y = recv.recv().unwrap();
                    if addr == 255 {
                        current_nat = Some((x, y));
                    } else {
                        my_sends[addr as usize].send(x).unwrap();
                        my_sends[addr as usize].send(y).unwrap();
                    }
                }
            }
            if all_idle {
                if let Some((x, y)) = current_nat {
                    if let Some(last_y) = last_y {
                        if last_y == y {
                            break last_y;
                        }
                    }

                    last_y = Some(y);
                    my_sends[0].send(x).unwrap();
                    my_sends[0].send(y).unwrap();
                } else {
                    unreachable!();
                }
            }
        }
    })
    .join()
    .unwrap()
}

fn part1(program: HashMap<i64, i64>) -> i64 {
    let mut my_sends = vec![];
    let mut my_recvs = vec![];
    for i in 0..50 {
        let (my_send, bot_recv) = channel();
        let (bot_send, my_recv) = channel();
        execute(program.clone(), bot_recv, bot_send);
        my_send.send(i).unwrap();
        my_sends.push(my_send);
        my_recvs.push(my_recv);
    }
    spawn(move || 'outer: loop {
        for recv in &my_recvs {
            if let Ok(addr) = recv.try_recv() {
                let x = recv.recv().unwrap();
                let y = recv.recv().unwrap();
                if addr == 255 {
                    break 'outer y;
                } else {
                    my_sends[addr as usize].send(x).unwrap();
                    my_sends[addr as usize].send(y).unwrap();
                }
            }
        }
    })
    .join()
    .unwrap()
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
