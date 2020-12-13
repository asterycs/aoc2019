use std::env;
use std::fs;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, VecDeque};

type Map = Vec<Vec<Tile>>;

fn build_map(input: &String) -> Map {
    let mut map = Map::new();

    for line in input.split('\n') {
        let mut row = Vec::new();
        for char in line.chars() {
            let tile = Tile::from(char);
            row.push(tile);
        }

        map.push(row);
    }

    map
}

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/18.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Vec2u {
    r: usize,
    c: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Entrance,
    Wall,
    Key(u8),
    Door(u8)
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Entrance,
            'A'..='Z' => Tile::Door(c as u8 - 65),
            'a'..='z' => Tile::Key(c as u8 - 97),
            _ => panic!("Unknown tile"),
        }
    }
}

fn find_entrance(map: &Map) -> Option<Vec2u> {
    for (row, line) in map.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            if *tile == Tile::Entrance {
                return Some(Vec2u{r: row, c: col});
            }
        }
    }

    None
}

fn get_neighbors(target: &Vec2u) -> [Vec2u; 4] {
    [Vec2u{r: target.r - 1, c: target.c}, Vec2u{r: target.r + 1, c: target.c}, Vec2u{r: target.r, c: target.c + 1}, Vec2u{r: target.r, c: target.c - 1}]
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MappingState {
    region: HashSet<Vec2u>,
    threads: Vec<Vec2u>,
    accessible_keys: HashMap<u8, Vec2u>
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ExplorationState {
    next_key: u8,
    current_position: Vec2u,
    distance_travelled: u32,
    obtained_keys: Vec<u8>,
    mapping_state: MappingState
}

impl Ord for ExplorationState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance_travelled.cmp(&self.distance_travelled)
    }
}

impl PartialOrd for ExplorationState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum TileStatus {
    Free,
    Inaccessible,
    AlreadyVisited,
    NewKey(u8),
    MissingKey
}

impl ExplorationState {
    fn new(next_key: u8, mapping_state: MappingState, start_position: &Vec2u) -> Self {
        ExplorationState{next_key: next_key, current_position: *start_position, distance_travelled: 0, obtained_keys: Vec::new(), mapping_state: mapping_state}
    }

    fn pick_up_key(&mut self, key: u8, map: &Map) {
        let key_position = *self.mapping_state.accessible_keys.get(&key).unwrap();
        self.mapping_state.accessible_keys.remove(&key);
        self.obtained_keys.push(key);

        let mut queue = vec![(self.current_position.clone(), 0u32)].into_iter().collect::<VecDeque<_>>();
        let mut visited = HashSet::new();

        while !queue.is_empty() {
            let (current_position, distance_from_start) = queue.pop_front().unwrap();

            if current_position == key_position {
                self.current_position = key_position;
                self.distance_travelled += distance_from_start;

                return;
            }

            let status = self.mapping_state.visit(&visited, &self.obtained_keys, &map, &current_position);

            match status {
                TileStatus::NewKey(_) | TileStatus::Free => {        
                    let neighbors = get_neighbors(&current_position);
                    queue.extend(&mut neighbors.iter().map(|n| (n.clone(), distance_from_start + 1)));
                },
                _ => ()
            }

            visited.insert(current_position);
        }
        
        panic!("Couldn't pick up key");
    }

    fn add_to_queue(&self, queue: &mut HashMap<(Vec<u8>, Vec2u), Self>) {
        let mut sorted_keys = self.obtained_keys.clone().into_iter().collect::<Vec<_>>();
        sorted_keys.sort();
        let key = (sorted_keys, self.current_position);

        // Check if there already is a state that has obtained the same keys with a lower distance
        match queue.get_mut(&key) {
            Some(other) => {
                if self.distance_travelled < other.distance_travelled {
                    *other = self.clone();
                }
            },
            None => {
                queue.insert(key.clone(), self.clone());
            }
        }
    }
}

impl MappingState {
    fn visit(&self, region: &HashSet<Vec2u>, obtained_keys: &Vec<u8>, map: &Map, target: &Vec2u) -> TileStatus {
        if region.contains(target) {
            return TileStatus::AlreadyVisited;
        }

        if let Some(row) = map.get(target.r) {
            if let Some(tile) = row.get(target.c) {
                match tile {
                    Tile::Door(d) => {
                        if obtained_keys.contains(&d) {
                            return TileStatus::Free;
                        }else{
                            return TileStatus::MissingKey;
                        }
                    },
                    Tile::Key(k) => {
                        if !obtained_keys.contains(k) {
                            return TileStatus::NewKey(*k);
                        }else{
                            return TileStatus::Free;
                        }
                    }
                    Tile::Empty | Tile::Entrance => return TileStatus::Free,
                    _ => (),
                } 
            }
        }

        return TileStatus::Inaccessible
    }

    fn expand(&mut self, map: &Map, obtained_keys: &Vec<u8>) {
        let mut queue = self.threads.drain(..).collect::<VecDeque<_>>();

        // bfs
        while !queue.is_empty() {
            let current_position = queue.pop_front().unwrap();
            let status = self.visit(&self.region, &obtained_keys, &map, &current_position);

            match status {
                TileStatus::MissingKey => {
                    self.threads.push(current_position);
                },
                TileStatus::Free | TileStatus::NewKey(_) => {
                    if let TileStatus::NewKey(k) = status {
                        self.accessible_keys.insert(k, current_position);
                    }

                    let neighbors = get_neighbors(&current_position);
                    queue.extend(neighbors.iter());
                    self.region.insert(current_position);
                },
                _ => ()
            }
        }
    }

    fn new(entrance: &Vec2u) -> Self {
        MappingState{region: HashSet::new(), threads: vec![*entrance], accessible_keys: HashMap::new()}
    }
}

fn part1(input: String) -> u32 {
    let map = build_map(&input);
    let entrance_position = find_entrance(&map).expect("No entrance?");

    let mut initial_mapping_state = MappingState::new(&entrance_position);
    initial_mapping_state.expand(&map, &Vec::new());
    let mut state_queue = Vec::new();
    let mut next_queue = HashMap::new();

    for key in initial_mapping_state.accessible_keys.iter() {
        state_queue.push(ExplorationState::new(*key.0, initial_mapping_state.clone(), &entrance_position));
    }

    let mut min_distance = std::u32::MAX;
    let mut obtained_keys = 0;

    while !state_queue.is_empty() {
        // Pick up the next key, prune if a cheaper state already was seen
        while !state_queue.is_empty() {
            let mut state = state_queue.pop().unwrap();

            if state.obtained_keys.len() > obtained_keys {
                obtained_keys = state.obtained_keys.len();
                println!("obtained keys: {}", state.obtained_keys.len());
            }

            state.pick_up_key(state.next_key, &map);
            state.mapping_state.expand(&map, &state.obtained_keys);

            if !state.mapping_state.accessible_keys.is_empty() {
                state.add_to_queue(&mut next_queue);
            }else{
                min_distance = std::cmp::min(min_distance, state.distance_travelled);
            }
        }

        // Expand the queued states
        for (_, state) in next_queue.drain().into_iter() {        
            for next_key in state.mapping_state.accessible_keys.iter() {
                let mut new_state = state.clone();
                new_state.next_key = *next_key.0;

                state_queue.push(new_state);
            }
        }
    }

    min_distance
}

fn main() {
    let input = get_input();

    let steps = part1(input);

    println!("steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1_test_input0() -> String {
        "#########\n#b.A.@.a#\n#########".to_owned()
    }

    #[test]
    fn part1_test0() {
        let input = part1_test_input0();

        let steps = part1(input);

        assert_eq!(steps, 8);
    }

    fn part1_test_input1() -> String {
        "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################".to_owned()
    }

    #[test]
    fn part1_test1() {
        let input = part1_test_input1();

        let steps = part1(input);

        assert_eq!(steps, 86);
    }

    fn part1_test_input2() -> String {
        "########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################".to_owned()
    }

    #[test]
    fn part1_test2() {
        let input = part1_test_input2();

        let steps = part1(input);

        assert_eq!(steps, 132);
    }

    fn part1_test_input3() -> String {
        "#################\n#i.G..c...e..H.p#\n########.########\n#j.A..b...f..D.o#\n########@########\n#k.E..a...g..B.n#\n########.########\n#l.F..d...h..C.m#\n#################".to_owned()
    }

    #[test]
    fn part1_test3() {
        let input = part1_test_input3();

        let steps = part1(input);

        assert_eq!(steps, 136);
    }
}
