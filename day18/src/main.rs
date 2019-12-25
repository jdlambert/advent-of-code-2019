use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Pos = (isize, isize);
type Map = HashMap<Pos, char>;
type Graph = HashMap<Node, HashSet<(Node, isize)>>;

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
enum Node {
    Start,
    Key(u8),
    Door(u8),
}

fn get_adjacent((i, j): Pos, map: &Map) -> HashSet<(Node, isize)> {
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
            _ => ()
        }
    }
    set
}

fn map_to_graph(map: Map) -> Graph {
    let mut graph = Graph::new();

    for (&pos, &tile) in &map {
        let node = match tile {
            '@' => Node::Start,
            c if c.is_ascii_uppercase() => Node::Door(c as u8 - b'A'),
            c if c.is_ascii_lowercase() => Node::Key(c as u8 - b'a'),
            _ => continue,
        };
        graph.insert(node, get_adjacent(pos, &map));
    }

    graph
}

fn part1(map: &Map) -> usize {
    println!("{:?}", map_to_graph(map.clone()));
    1
}

// #[derive(Hash, Debug, Clone, PartialEq, Eq)]
// struct MultiState {
//     pos: Vec<Pos>,
//     keys: u32,
// }

// // This is too slow too solve the actual problem in a reasonable timeframe, but it correctly solves the test cases.
// // TODO: Process the map into a graph, compressing the "Open" tiles into edges with length

// fn part2(map: &mut HashMap<Pos, Tile>, (i, j): Pos) -> usize {
//     let mut queue = VecDeque::new();
//     let mut visited = HashMap::new();

//     let pos = vec![
//         (i + 1, j + 1),
//         (i + 1, j - 1),
//         (i - 1, j + 1),
//         (i - 1, j - 1),
//     ];

//     map.remove(&(i, j)).unwrap();
//     map.remove(&(i + 1, j)).unwrap();
//     map.remove(&(i - 1, j)).unwrap();
//     map.remove(&(i, j + 1)).unwrap();
//     map.remove(&(i, j - 1)).unwrap();

//     queue.push_back(MultiState {
//         pos: pos.clone(),
//         keys: 0,
//     });
//     visited.insert(
//         MultiState {
//             pos: pos.clone(),
//             keys: 0,
//         },
//         0,
//     );

//     loop {
//         if let Some(state) = queue.pop_front() {
//             let steps = *visited.get(&state).unwrap();
//             if state.keys == 0x3FFFFFF {
//                 // 26 ones for 26 letters
//                 break steps;
//             } else {
//                 for (i, &pos) in state.pos.iter().enumerate() {
//                     let bot_state = State {
//                         pos,
//                         keys: state.keys,
//                     };
//                     for next in bot_state.next_states(&map) {
//                         let mut new_positions = state.pos.clone();
//                         new_positions[i] = next.pos;
//                         let next_state = MultiState {
//                             pos: new_positions,
//                             keys: next.keys,
//                         };
//                         if !visited.contains_key(&next_state) {
//                             queue.push_back(next_state.clone());
//                             visited.insert(next_state.clone(), steps + 1);
//                         }
//                     }
//                 }
//             }
//         } else {
//             unreachable!();
//         }
//     }
// }

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
    // println!("Part 2: {}", part2(&mut map, start));
}
