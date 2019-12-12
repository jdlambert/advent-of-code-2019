use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};
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
                    program.insert(index, input.recv().unwrap());
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

fn paint(data: HashMap<i64, i64>, initial_color: i64) -> HashMap<(i64, i64), i64> {
    let (bot_send, my_recv) = channel();
    let (my_send, bot_recv) = channel();
    let bot_handle = execute(data.clone(), bot_recv, bot_send);
    let paint_handle = spawn(move || {
        let mut pos = (0, 0);
        let mut direction = (0, -1);
        let mut painted = HashMap::new();
        painted.insert(pos, initial_color);
        loop {
            let spot = painted.get(&pos);
            if let Some(color) = spot {
                my_send.send(*color).unwrap_or(());
            } else {
                my_send.send(0).unwrap_or(());
            }
            match my_recv.recv() {
                Ok(color) => painted.insert(pos, color),
                Err(_) => return painted,
            };
            direction = match my_recv.recv() {
                Ok(0) => match direction {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!("Bad direction!"),
                },
                Ok(1) => match direction {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => panic!("Bad direction!"),
                },
                Err(_) => return painted,
                _ => panic!("Bad direction!"),
            };
            pos = (pos.0 + direction.0, pos.1 + direction.1);
        }
    });
    bot_handle.join().unwrap();
    paint_handle.join().unwrap()
}

fn part1(data: &HashMap<i64, i64>) -> usize {
    paint(data.clone(), 0).len()
}

fn part2(data: &HashMap<i64, i64>) -> String {
    let painted = paint(data.clone(), 1);
    let xs = painted.keys().map(|(x, _)| *x);
    let ys = painted.keys().map(|(_, y)| *y);
    let max_x = xs.clone().max().unwrap() + 1;
    let min_x = xs.min().unwrap();
    let max_y = ys.clone().max().unwrap() + 1;
    let min_y = ys.min().unwrap();
    let mut result = String::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            result.push(if painted.get(&(x, y)).unwrap_or(&0) > &0 {
                '#'
            } else { ' ' });
        }
        result.push('\n');
    }
    result
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let data: HashMap<i64, i64> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, v)| (i as i64, v))
        .collect();

    println!("Part 1: {}", part1(&data));
    println!("Part 2: \n{}", part2(&data));
}
