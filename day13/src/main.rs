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

fn part1(data: &HashMap<i64, i64>) -> usize {
    let (bot_send, my_recv) = channel();
    let (_my_send, bot_recv) = channel();
    execute(data.clone(), bot_recv, bot_send);
    let draw_handle = spawn(move || {
        let mut tiles = HashMap::new();
        loop {
            let (x, y, t) = match (my_recv.recv(), my_recv.recv(), my_recv.recv()) {
                (Ok(i), Ok(j), Ok(u)) => (i, j, u),
                _ => return tiles,
            };
            tiles.insert((x, y), t);
        }
    });
    let tiles = draw_handle.join().unwrap();
    tiles.iter().filter(|(_, tile)| **tile == 2).count()
}

fn _render(tiles: &HashMap<(i64, i64), i64>) -> String {
    let xs = tiles.keys().map(|(x, _)| *x);
    let ys = tiles.keys().map(|(_, y)| *y);
    let max_x = xs.clone().max().unwrap() + 1;
    let min_x = xs.min().unwrap();
    let max_y = ys.clone().max().unwrap() + 1;
    let min_y = ys.min().unwrap();
    let mut result = String::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            result.push(match tiles.get(&(x, y)).unwrap_or(&0) {
                0 => ' ',
                1 => '#',
                2 => '_',
                3 => '=',
                4 => 'o',
                _ => panic!("Unknown tile!"),
            });
        }
        result.push('\n');
    }
    result
}

fn get_tile(tiles: &HashMap<(i64, i64), i64>, target: i64) -> Option<(i64, i64)> {
    match tiles.iter().filter(|(_, m)| **m == target).count() {
        0 => None,
        _ => Some(
            *tiles
                .iter()
                .filter(|(_, m)| **m == target)
                .map(|(pos, _)| *pos)
                .collect::<Vec<(i64, i64)>>()
                .first()
                .unwrap(),
        ),
    }
}
fn part2(data: &HashMap<i64, i64>) -> i64 {
    let (bot_send, my_recv) = channel();
    let (my_send, bot_recv) = channel();
    my_send.send(0).unwrap_or(());
    execute(data.clone(), bot_recv, bot_send);
    let play_handle = spawn(move || {
        let mut tiles = HashMap::new();
        let mut score = 0;
        loop {
            match my_recv.recv() {
                Ok(-1) => {
                    my_recv.recv().unwrap();
                    score = my_recv.recv().unwrap();
                }
                Ok(x) => {
                    let y = my_recv.recv().unwrap();
                    let tile = my_recv.recv().unwrap();
                    tiles.insert((x, y), tile);
                    if tile == 4 {
                        let (paddle, ball) = (get_tile(&tiles, 3), get_tile(&tiles, 4));
                        if let (Some((paddle_x, _)), Some((ball_x, _))) = (paddle, ball) {
                            let val = if paddle_x > ball_x {
                                -1
                            } else if paddle_x < ball_x {
                                1
                            } else {
                                0
                            };
                            my_send.send(val).unwrap_or(());
                        }
                    }
                }
                _ => return score,
            }
            // println!("{}", render(&tiles));
        }
    });
    play_handle.join().unwrap()
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

    println!("Part 1: {}", part1(&data));
    data.insert(0, 2);
    println!("Part 2: {}", part2(&data));
}
