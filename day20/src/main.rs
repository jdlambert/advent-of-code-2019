use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

type Pos = (isize, isize);
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Portal {
    name: String,
    outer: bool,
}
type Map = HashMap<Pos, char>;
type Graph = HashMap<Portal, HashSet<(Portal, usize)>>;

fn get_adjacent((i, j): Pos, map: &Map) -> HashSet<(Portal, usize)> {
    let mut set = HashSet::new();
    let mut seen = HashSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(((i + 1, j), 1));
    frontier.push_back(((i - 1, j), 1));
    frontier.push_back(((i, j + 1), 1));
    frontier.push_back(((i, j - 1), 1));
    seen.insert((i, j));

    while frontier.len() > 0 {
        let ((i, j), steps) = frontier.pop_front().unwrap();
        if seen.contains(&(i, j)) {
            continue;
        }
        seen.insert((i, j));
        match map.get(&(i, j)).unwrap_or(&'#') {
            '.' => {
                frontier.push_back(((i + 1, j), steps + 1));
                frontier.push_back(((i - 1, j), steps + 1));
                frontier.push_back(((i, j + 1), steps + 1));
                frontier.push_back(((i, j - 1), steps + 1));
            }
            c if c.is_ascii_uppercase() => {
                if let Some(node) = get_portal((i, j), &map) {
                    set.insert((node, steps - 1));
                }
            }
            _ => (),
        }
    }
    set
}

fn get_portal((i, j): Pos, map: &Map) -> Option<Portal> {
    let mut name = String::new();
    let mut outer = false;

    if *map.get(&(i - 1, j)).unwrap_or(&' ') == '.' {
        name.push(*map.get(&(i, j)).unwrap());
        name.push(*map.get(&(i + 1, j)).unwrap());
        outer = map.get(&(i + 2, j)).is_none();
    } else if *map.get(&(i + 1, j)).unwrap_or(&' ') == '.' {
        name.push(*map.get(&(i - 1, j)).unwrap());
        name.push(*map.get(&(i, j)).unwrap());
        outer = map.get(&(i - 2, j)).is_none();
    } else if *map.get(&(i, j - 1)).unwrap_or(&' ') == '.' {
        name.push(*map.get(&(i, j)).unwrap());
        name.push(*map.get(&(i, j + 1)).unwrap());
        outer = map.get(&(i, j + 2)).is_none();
    } else if *map.get(&(i, j + 1)).unwrap_or(&' ') == '.' {
        name.push(*map.get(&(i, j - 1)).unwrap());
        name.push(*map.get(&(i, j)).unwrap());
        outer = map.get(&(i, j - 2)).is_none();
    }

    if name.len() > 0 {
        Some(Portal { name, outer })
    } else {
        None
    }
}

fn map_to_graph(map: Map) -> Graph {
    let mut graph = Graph::new();

    for (&pos, &c) in &map {
        if c.is_ascii_uppercase() {
            if let Some(Portal { name, outer }) = get_portal(pos, &map) {
                let set = graph
                    .entry(Portal {
                        name: name.clone(),
                        outer,
                    })
                    .or_insert(HashSet::new());
                for adjacent in get_adjacent(pos, &map) {
                    set.insert(adjacent);
                }
                if name != "AA" && name != "ZZ" {
                    set.insert((
                        Portal {
                            name: name.clone(),
                            outer: !outer,
                        },
                        0,
                    ));
                }
            }
        }
    }

    graph
}

#[derive(Eq, PartialEq)]
struct State {
    portal: Portal,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(graph: Graph) -> usize {
    let mut seen = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        portal: Portal {
            name: "AA".to_owned(),
            outer: true,
        },
        cost: 0,
    });

    while let Some(State { portal, cost }) = frontier.pop() {
        if seen.contains(&portal) {
            continue;
        }
        seen.insert(portal.clone());

        if portal.name == "ZZ" {
            return cost;
        }

        for (adjacent, len) in graph.get(&portal).unwrap() {
            frontier.push(State {
                portal: adjacent.to_owned(),
                cost: cost + len,
            });
        }
    }

    return std::usize::MAX;
}

fn part1(map: &Map) -> usize {
    let graph = map_to_graph(map.clone());
    shortest_path(graph) - 1
}

fn part2(map: &Map) -> usize {
    2
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();

    let mut map = Map::new();

    for (j, line) in content.lines().enumerate() {
        let j = j as isize;
        for (i, c) in line.chars().enumerate() {
            let i = i as isize;
            map.insert((i, j), c);
        }
    }

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
