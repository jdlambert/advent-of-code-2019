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

type Map = HashMap<(i64, i64), i64>;

fn part1(map: &Map) -> usize {
    bfs_iter(map, (0, 0))
        .take_while(|frontier| !frontier.into_iter().any(|pos| map.get(&pos) == Some(&2)))
        .count()
}

fn part2(map: &Map) -> usize {
    let (oxygen_pos, _) = map.iter().find(|(_, &status)| status == 2).unwrap();

    bfs_iter(map, *oxygen_pos).count() - 1
}

fn bfs_iter(map: &Map, start: (i64, i64)) -> impl Iterator<Item = Vec<(i64, i64)>> + '_ {
    let mut visited = HashSet::new();

    std::iter::successors(Some(vec![start]), move |frontier| {
        let mut next_frontier = Vec::new();
        for &(x, y) in frontier {
            visited.insert((x, y));

            let adjacent = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

            for next_pos in &adjacent {
                if !visited.contains(next_pos) {
                    if let Some(&s) = map.get(&next_pos) {
                        if s > 0 {
                            next_frontier.push(*next_pos);
                        }
                    }
                }
            }
        }

        if next_frontier.is_empty() {
            None
        } else {
            Some(next_frontier)
        }
    })
}

fn step_dir(dir: (i64, i64), sender: &Sender<i64>, receiver: &Receiver<i64>) -> i64 {
    sender
        .send(match dir {
            (0, -1) => 1,
            (0, 1) => 2,
            (-1, 0) => 3,
            (1, 0) => 4,
            _ => unreachable!(),
        })
        .unwrap_or(());
    receiver.recv().unwrap_or(0)
}

fn explore_point(
    (x, y): (i64, i64),
    map: &mut Map,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) {
    for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_pos = (x + dx, y + dy);
        if !map.contains_key(&new_pos) {
            let status = step_dir((dx, dy), sender, receiver);
            map.insert(new_pos, status);
            if status != 0 {
                explore_point(new_pos, map, sender, receiver);
                step_dir((-dx, -dy), sender, receiver);
            }
        }
    }
}

fn explore_map(program: HashMap<i64, i64>) -> Map {
    let (my_send, bot_recv) = channel();
    let (bot_send, my_recv) = channel();

    execute(program, bot_recv, bot_send);

    let mut map = Map::new();
    map.insert((0, 0), 1);
    explore_point((0, 0), &mut map, &my_send, &my_recv);
    map
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

    let map = explore_map(program);

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
