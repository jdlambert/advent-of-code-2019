use std::{
    collections::{HashMap, VecDeque},
    fs,
};

type Pos = (isize, isize);

#[derive(Debug)]
enum Tile {
    Open,
    Key(u8),
    Door(u8),
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    pos: Pos,
    keys: u32,
}

const DIRS: [Pos; 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

impl State {
    fn next_states(&self, map: &HashMap<Pos, Tile>) -> Vec<State> {
        DIRS.into_iter()
            .filter_map(|(dx, dy)| {
                let pos = (self.pos.0 + dx, self.pos.1 + dy);

                match map.get(&pos) {
                    Some(Tile::Door(key)) => {
                        if self.keys & (1 << key) as u32 > 0 {
                            Some(State {
                                keys: self.keys,
                                pos,
                            })
                        } else {
                            None
                        }
                    }
                    Some(Tile::Key(key)) => Some(State {
                        keys: self.keys | (1 << key) as u32,
                        pos,
                    }),
                    Some(Tile::Open) => Some(State {
                        keys: self.keys,
                        pos,
                    }),
                    _ => None,
                }
            })
            .collect()
    }
}

// This is very slow and could some major refactoring

fn part1(map: &HashMap<Pos, Tile>, pos: Pos) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back(State { pos, keys: 0 });
    visited.insert(State { pos, keys: 0 }, 0);

    loop {
        if let Some(state) = queue.pop_front() {
            let steps = *visited.get(&state).unwrap();
            if state.keys == 0x3FFFFFF {
                // 26 ones for 26 letters
                break steps;
            } else {
                for next in state.next_states(&map) {
                    if !visited.contains_key(&next) {
                        queue.push_back(next.clone());
                        visited.insert(next.clone(), steps + 1);
                    }
                }
            }
        } else {
            unreachable!();
        }
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct MultiState {
    pos: Vec<Pos>,
    keys: u32,
}

// This is too slow too solve the actual problem in a reasonable timeframe, but it correctly solves the test cases.
// TODO: Process the map into a graph, compressing the "Open" tiles into edges with length

fn part2(map: &mut HashMap<Pos, Tile>, (i, j): Pos) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    let pos = vec![
        (i + 1, j + 1),
        (i + 1, j - 1),
        (i - 1, j + 1),
        (i - 1, j - 1),
    ];

    map.remove(&(i, j)).unwrap();
    map.remove(&(i + 1, j)).unwrap();
    map.remove(&(i - 1, j)).unwrap();
    map.remove(&(i, j + 1)).unwrap();
    map.remove(&(i, j - 1)).unwrap();

    queue.push_back(MultiState {
        pos: pos.clone(),
        keys: 0,
    });
    visited.insert(
        MultiState {
            pos: pos.clone(),
            keys: 0,
        },
        0,
    );

    loop {
        if let Some(state) = queue.pop_front() {
            let steps = *visited.get(&state).unwrap();
            if state.keys == 0x3FFFFFF {
                // 26 ones for 26 letters
                break steps;
            } else {
                for (i, &pos) in state.pos.iter().enumerate() {
                    let bot_state = State {
                        pos,
                        keys: state.keys,
                    };
                    for next in bot_state.next_states(&map) {
                        let mut new_positions = state.pos.clone();
                        new_positions[i] = next.pos;
                        let next_state = MultiState {
                            pos: new_positions,
                            keys: next.keys,
                        };
                        if !visited.contains_key(&next_state) {
                            queue.push_back(next_state.clone());
                            visited.insert(next_state.clone(), steps + 1);
                        }
                    }
                }
            }
        } else {
            unreachable!();
        }
    }
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let mut map = HashMap::new();
    let mut start = None;

    for (j, line) in content.lines().enumerate() {
        let j = j as isize;
        for (i, c) in line.chars().enumerate() {
            let i = i as isize;
            match c {
                '.' => {
                    map.insert((i, j), Tile::Open);
                }
                '@' => {
                    start = Some((i, j));
                    map.insert((i, j), Tile::Open);
                }
                'a'..='z' => {
                    map.insert((i, j), Tile::Key(c as u8 - b'a'));
                }
                'A'..='Z' => {
                    map.insert((i, j), Tile::Door(c as u8 - b'A'));
                }
                _ => (),
            }
        }
    }
    let start = start.unwrap();

    println!("Part 1: {}", part1(&map, start));
    println!("Part 2: {}", part2(&mut map, start));
}
