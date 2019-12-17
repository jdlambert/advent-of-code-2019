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

type Map = HashMap<(i32, i32), char>;

fn part1(map: &Map) -> i32 {



    map.iter()
        .filter(|(&(x, y), &v)| {
            v == '#'
                && *map.get(&(x + 1, y)).unwrap_or(&' ') == '#'
                && *map.get(&(x - 1, y)).unwrap_or(&' ') == '#'
                && *map.get(&(x, y + 1)).unwrap_or(&' ') == '#'
                && *map.get(&(x, y - 1)).unwrap_or(&' ') == '#'
        })
        .inspect(|(pos, _)| println!("{:?}", pos))
        .map(|((x, y), _)| x * y)
        .sum()
}

fn part2(program: HashMap<i64, i64>) -> i64 {
    let (my_send, bot_recv) = channel();
    let (bot_send, my_recv) = channel();

    let bot_handle = execute(program, bot_recv, bot_send);
    
    const IN: &str = "A,B,A,B,A,C,B,C,A,C\nR,4,L,10,L,10\nL,8,R,12,R,10,R,4\nL,8,L,8,R,10,R,4\nn\n";

    for byte in IN.as_bytes() {
        my_send.send(*byte as i64).unwrap()
    }
    bot_handle.join();
    while let Ok(val) = my_recv.recv() {
        if (val > 127) {
            return val
        }
    }
    return 0;
}

fn camera_view(program: HashMap<i64, i64>) -> Map {
    let (_, bot_recv) = channel();
    let (bot_send, my_recv) = channel();

    let bot_handle = execute(program, bot_recv, bot_send);

    let mut map = Map::new();
    let mut i = 0;
    let mut j = 0;
    while let Ok(val) = my_recv.recv() {
        let c = char::from(val as u8);
        map.insert((i, j), c);
        print!("{}", c);
        if c == '\n' {
            i = 0;
            j += 1;
        } else {
            i += 1;
        }
    }
    map
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

    let map = camera_view(program.clone());

    println!("Part 1: {}", part1(&map));
    program.insert(0, 2);
    println!("Part 2: {}", part2(program.clone()));
}
