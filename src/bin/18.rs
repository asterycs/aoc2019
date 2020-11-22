use std::env;
use std::fs;
use std::path::PathBuf;
use std::collections::{HashSet, HashMap, VecDeque};

type MapData = Vec<Vec<Tile>>;
struct Map {
    map: MapData,
    keys: HashMap<u32, Vec2u>,
    doors: HashMap<u32, Vec2u>
}

fn build_map(input: &String) -> Map {
    let mut map = MapData::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();

    for (r, line) in input.split('\n').enumerate() {
        let mut row = Vec::new();
        for (c, char) in line.chars().enumerate() {
            let tile = Tile::from(char);
            let coordinate  = Vec2u{r, c};

            if let Tile::Key(n) = tile {
                keys.insert(n, coordinate);
            }else if let Tile::Door(n) = tile {
                doors.insert(n, coordinate);
            }

            row.push(tile);
        }

        map.push(row);
    }

    Map{map, keys, doors}
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
    Key(u32),
    Door(u32)
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Entrance,
            'A'..='Z' => Tile::Door(c as u32 - 65),
            'a'..='z' => Tile::Key(c as u32 - 97),
            _ => panic!("Unknown tile"),
        }
    }
}

fn find_entrance(map: &MapData) -> Option<Vec2u> {
    for (row, line) in map.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            if *tile == Tile::Entrance {
                return Some(Vec2u{r: row, c: col});
            }
        }
    }

    None
}

fn get_neighbors(target: &Vec2u) -> Vec<Vec2u> {
    let neighbors = [Vec2u{r: target.r - 1, c: target.c}, Vec2u{r: target.r + 1, c: target.c}, Vec2u{r: target.r, c: target.c + 1}, Vec2u{r: target.r, c: target.c - 1}];
    neighbors.to_vec()
}

struct Expansion {
    accessible: Vec<Vec2u>,
    blocked_by_door: Vec<Vec2u>
}

#[derive(Debug, Clone)]
struct SearchState {
    current_position: Vec2u,
    distance_travelled: u32,
    region: HashSet<Vec2u>,
    threads: HashSet<Vec2u>, // Where to pick up searching
    accessible_keys: HashMap<u32, Vec2u>,
    obtained_keys: HashSet<u32>
}

#[derive(Debug)]
enum TileStatus {
    Free,
    Inaccessible,
    AlreadyVisited,
    NewKey(u32),
    MissingKey
}

impl SearchState {
    fn visit(&self, region: &HashSet<Vec2u>, map: &MapData, target: &Vec2u) -> TileStatus {
        if region.contains(target) {
            return TileStatus::AlreadyVisited;
        }


        if let Some(row) = map.get(target.r) {
            if let Some(tile) = row.get(target.c) {
                match tile {
                    Tile::Door(d) => {
                        if self.obtained_keys.contains(&d) {
                            return TileStatus::Free;
                        }else{
                            return TileStatus::MissingKey;
                        }
                    },
                    Tile::Key(k) => {
                        if !self.obtained_keys.contains(k) {
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

    fn expand(&mut self, map: &Map) {
        let mut stack = self.threads.drain().collect::<VecDeque<_>>();

        // bfs
        while !stack.is_empty() {
            let current_position = stack.pop_front().unwrap();
            let status = self.visit(&self.region, &map.map, &current_position);

            match status {
                TileStatus::MissingKey => {
                    self.threads.insert(current_position);
                },
                TileStatus::NewKey(k) => {
                    self.accessible_keys.insert(k, current_position);
                    // WETWET
                    let neighbors = get_neighbors(&current_position);
                    stack.append(&mut neighbors.into_iter().collect());
                    self.region.insert(current_position);
                }
                TileStatus::Free => {
                    // WETWET
                    let neighbors = get_neighbors(&current_position);
                    stack.append(&mut neighbors.into_iter().collect());
                    self.region.insert(current_position);
                },
                _ => ()
            }
        }
    }

    fn new(entrance: &Vec2u) -> Self {
        SearchState{current_position: *entrance, distance_travelled: 0, region: HashSet::new(), threads: vec![*entrance].into_iter().collect(), accessible_keys: HashMap::new(), obtained_keys: HashSet::new()}
    }

    fn pick_up_key(&mut self, key: u32, map: &MapData, position: &mut Vec2u) -> u32 {
        let key_position = *self.accessible_keys.get(&key).unwrap();
        self.accessible_keys.remove(&key);
        self.obtained_keys.insert(key);

        let mut stack = vec![(position.clone(), 0u32)].into_iter().collect::<VecDeque<_>>();
        let mut visited = HashSet::new();

        while !stack.is_empty() {
            let (current_position, distance_from_start) = stack.pop_front().unwrap();

            if current_position == key_position {
                *position = key_position;
                return distance_from_start;
            }

            let status = self.visit(&visited, &map, &current_position);

            match status {
                TileStatus::NewKey(_) | TileStatus::Free => {        
                    let neighbors = get_neighbors(&current_position);
                    stack.append(&mut neighbors.into_iter().map(|n| (n, distance_from_start + 1)).collect());
                },
                _ => ()
            }

            visited.insert(current_position);
        }
        
        panic!("Couldn't pick up key");
    }
}

fn choose_next_key(map: &MapData, state: &SearchState, &position: &Vec2u) -> u32 {
    *state.accessible_keys.iter().next().unwrap().0
}

fn part1(input: String) -> u32 {
    let map = build_map(&input);
    let mut current_position = find_entrance(&map.map).expect("No entrance?");

    let mut states = vec![SearchState::new(&current_position)].into_iter().collect::<VecDeque<_>>();

    while !state.threads.is_empty(){
        state.expand(&map);

        if !state.accessible_keys.is_empty() {
            if state.accessible_keys.len() > 1 {
                panic!("So many choices!");
            }
    
            let key = choose_next_key(&map.map, &state, &current_position);
            let distance_to_key = state.pick_up_key(key, &map.map, &mut current_position);

            distance_travelled += distance_to_key;
        }else{
            panic!("Blocking doors but no keys are accessible");
        }
    }

    distance_travelled
}

fn main() {
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
}
