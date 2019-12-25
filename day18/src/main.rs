use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

type Pos = (isize, isize);
type Map = HashMap<Pos, char>;
type Graph = HashMap<Node, HashSet<(Node, usize)>>;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Start(u8),
    Key(u8),
    Door(u8),
}

fn get_adjacent((i, j): Pos, map: &Map) -> HashSet<(Node, usize)> {
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
            '.' | '@' => {
                frontier.push_back(((i + 1, j), steps + 1));
                frontier.push_back(((i - 1, j), steps + 1));
                frontier.push_back(((i, j + 1), steps + 1));
                frontier.push_back(((i, j - 1), steps + 1));
            }
            c if c.is_ascii_lowercase() => {
                set.insert((Node::Key(*c as u8 - b'a'), steps));
            }
            c if c.is_ascii_uppercase() => {
                set.insert((Node::Door(*c as u8 - b'A'), steps));
            }
            _ => (),
        }
    }
    set
}

fn map_to_graph(map: Map) -> Graph {
    let mut graph = Graph::new();
    let mut starts_seen = 0;

    for (&pos, &tile) in &map {
        let node = match tile {
            '@' => {
                let start = Node::Start(starts_seen);
                starts_seen += 1;
                start
            }
            c if c.is_ascii_uppercase() => Node::Door(c as u8 - b'A'),
            c if c.is_ascii_lowercase() => Node::Key(c as u8 - b'a'),
            _ => continue,
        };
        graph.insert(node, get_adjacent(pos, &map));
    }

    graph
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct State {
    bots: Vec<Node>,
    keys: u32,
}

impl State {
    fn key_count(&self) -> u32 {
        let mut keys = self.keys;
        let mut count = 0;
        while keys > 0 {
            count += (keys & 1);
            keys >>= 1;
        }
        count
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.key_count().cmp(&other.key_count())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct CostState {
    state: State,
    cost: usize,
}

impl Ord for CostState {
    fn cmp(&self, other: &CostState) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl PartialOrd for CostState {
    fn partial_cmp(&self, other: &CostState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_successors(graph: &Graph, state: &State, cost: usize) -> HashSet<CostState> {
    let mut successors = HashSet::new();
    for (i, bot) in state.bots.iter().enumerate() {
        let adjacents = graph.get(bot).unwrap();
        for (adjacent, len) in adjacents {
            match adjacent {
                Node::Door(key) => {
                    if state.keys & (1 << key) as u32 > 0 {
                        let mut bots = state.bots.clone();
                        bots[i] = adjacent.clone();
                        let state = State {
                            keys: state.keys,
                            bots,
                        };
                        let cost = cost + *len;
                        successors.insert(CostState { state, cost });
                    }
                }
                Node::Key(key) => {
                    let keys = state.keys | (1 << key) as u32;
                    let mut bots = state.bots.clone();
                    bots[i] = adjacent.clone();
                    let state = State { keys, bots };
                    let cost = cost + *len;
                    successors.insert(CostState { state, cost });
                }
                _ => unreachable!(),
            }
        }
    }
    successors
}

fn shortest_path(graph: Graph, states: BinaryHeap<CostState>) -> usize {
    let mut states = states.clone();
    let mut seen = HashSet::new();

    while let Some(CostState { state, cost }) = states.pop() {
        if state.key_count() == 26 {
            return cost;
        }
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());

        for successor in get_successors(&graph, &state, cost) {
            states.push(successor);
        }
    }

    return std::usize::MAX;
}

fn part1(map: &Map) -> usize {
    let graph = map_to_graph(map.clone());
    let mut states = BinaryHeap::new();
    let bots = vec![Node::Start(0)];
    let keys = 0;
    let state = State { bots, keys };
    let cost_state = CostState { state, cost: 0 };
    states.push(cost_state);
    shortest_path(graph, states)
}

fn part2(map: &Map) -> usize {
    let ((i, j), _) = map.iter().find(|(_, &c)| c == '@').unwrap();

    let mut map = map.clone();
    for di in -1..=1 {
        for dj in -1..=1 {
            map.insert((i + di, j + dj), if di == 0 || dj == 0 { '#' } else { '@' });
        }
    }

    let graph = map_to_graph(map);
    let mut states = BinaryHeap::new();
    for bot in 0..5 {
        let bots = vec![Node::Start(bot)];
        let keys = 0;
        let state = State { bots, keys };
        let cost_state = CostState { state, cost: 0 };
        states.push(cost_state);
    }

    shortest_path(graph, states)
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
    println!("Part 2: {}", part2(&mut map));
}
