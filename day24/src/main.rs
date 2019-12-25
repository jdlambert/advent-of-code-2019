use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Env {
    pub state: u32,
}

impl Env {
    pub fn new() -> Self {
        Env { state: 0 }
    }

    pub fn get(&self, (i, j): (isize, isize)) -> u32 {
        if i < 0 || i > 4 || j < 0 || j > 4 {
            return 0;
        }
        let index = i + 5 * j;
        (self.state & (1 << index)) >> index
    }

    pub fn next(&self) -> Self {
        let mut state = self.state;
        for i in 0..5 {
            for j in 0..5 {
                let current = self.get((i, j));
                let adjacent = self.get((i - 1, j))
                    + self.get((i + 1, j))
                    + self.get((i, j - 1))
                    + self.get((i, j + 1));
                if current == 0 && (adjacent == 2 || adjacent == 1) {
                    state |= 1 << (i + 5 * j);
                } else if current == 1 && adjacent != 1 {
                    state &= !(1 << (i + 5 * j));
                }
            }
        }
        Env { state }
    }
}

fn part1(env: Env) -> u32 {
    let mut seen = HashSet::new();
    let mut env = env;
    loop {
        seen.insert(env);
        env = env.next();
        if seen.contains(&env) {
            break env.state;
        }
    }
}

#[derive(Debug, Clone)]
struct RecursiveEnv {
    pub envs: HashMap<isize, u32>,
}

impl RecursiveEnv {
    pub fn from_env(env: Env) -> Self {
        let mut envs = HashMap::new();
        envs.insert(0, env.state);
        envs.insert(1, 0);
        envs.insert(-1, 0);
        RecursiveEnv { envs }
    }

    pub fn get(&self, level: isize, from: (isize, isize), (i, j): (isize, isize)) -> u32 {
        if i < 0 {
            self.get(level + 1, from, (1, 2))
        } else if i > 4 {
            self.get(level + 1, from, (3, 2))
        } else if j < 0 {
            self.get(level + 1, from, (2, 1))
        } else if j > 4 {
            self.get(level + 1, from, (2, 3))
        } else if (i, j) == (2, 2) {
            match from {
                (1, 2) => (0..5).map(|v| self.get(level - 1, from, (0, v))).sum(),
                (3, 2) => (0..5).map(|v| self.get(level - 1, from, (4, v))).sum(),
                (2, 1) => (0..5).map(|v| self.get(level - 1, from, (v, 0))).sum(),
                (2, 3) => (0..5).map(|v| self.get(level - 1, from, (v, 4))).sum(),
                _ => unreachable!(),
            }
        } else {
            let index = i + 5 * j;
            let state = self.envs.get(&level).unwrap_or(&0);
            (state & (1 << index)) >> index
        }
    }

    pub fn next(&self) -> Self {
        let mut envs = HashMap::new();

        for (level, state) in self.envs.clone() {
            let mut state = state;
            for i in 0..5 {
                for j in 0..5 {
                    if (i, j) == (2, 2) {
                        continue;
                    }
                    let current = self.get(level, (i, j), (i, j));
                    let adjacent = self.get(level, (i, j), (i - 1, j))
                        + self.get(level, (i, j), (i + 1, j))
                        + self.get(level, (i, j), (i, j - 1))
                        + self.get(level, (i, j), (i, j + 1));
                    if current == 0 && (adjacent == 2 || adjacent == 1) {
                        state |= 1 << (i + 5 * j);
                    } else if current == 1 && adjacent != 1 {
                        state &= !(1 << (i + 5 * j));
                    }
                }
            }
            envs.insert(level, state);
        }
        let min_level = envs.keys().min().unwrap();
        if *envs.get(min_level).unwrap() > 0 {
            envs.insert(min_level - 1, 0);
        }
        let max_level = envs.keys().max().unwrap();
        if *envs.get(max_level).unwrap() > 0 {
            envs.insert(max_level + 1, 0);
        }
        RecursiveEnv { envs }
    }

    fn bugs(&self) -> u32 {
        let mut count = 0;
        for (_, state) in self.envs.clone() {
            let mut state = state;
            while state > 0 {
                count += state & 1;
                state >>= 1;
            }
        }
        count
    }
}

fn part2(env: Env, rounds: usize) -> u32 {
    let mut env = RecursiveEnv::from_env(env);
    for i in 0..rounds {
        env = env.next();
    }
    env.bugs()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut env = Env::new();
    let mut i = 0;
    for ch in content.chars() {
        if ch == '\n' {
            continue;
        }
        if ch == '#' {
            env.state |= 1 << i;
        }
        i += 1;
    }
    println!("Part 1: {}", part1(env));
    println!("Part 2: {}", part2(env, 200));
}
