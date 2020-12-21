use std::env;
use std::fs;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

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

fn find_entrances(map: &Map) -> Vec<Vec2u> {
    let mut entrances = Vec::new();

    for (row, line) in map.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            if *tile == Tile::Entrance {
                entrances.push(Vec2u{r: row, c: col});
            }
        }
    }

    entrances
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
    current_position: Vec2u,
    distance_travelled: u32,
    obtained_keys: Vec<u8>,
    mapping_state: MappingState
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ExplorationTask {
    next_key: u8,
    explorer: ExplorationState
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
    fn new(mapping_state: MappingState, start_position: &Vec2u) -> Self {
        ExplorationState{current_position: *start_position, distance_travelled: 0, obtained_keys: Vec::new(), mapping_state: mapping_state}
    }

    fn pick_up_key(&mut self, key: u8, map: &Map, obtained_keys: &Vec<u8>) {
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

            let status = self.mapping_state.visit(&visited, &obtained_keys, &map, &current_position);

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

    fn new(start_position: Vec2u) -> Self {
        MappingState{region: HashSet::new(), threads: vec![start_position], accessible_keys: HashMap::new()}
    }
}

fn part1(input: String) -> u32 {
    let map = build_map(&input);
    let entrance_position = find_entrances(&map);

    let mut initial_mapping_state = MappingState::new(entrance_position.first().unwrap().clone());
    initial_mapping_state.expand(&map, &Vec::new());
    let mut task_queue = Vec::new();
    let mut next_queue = HashMap::new();

    for key in initial_mapping_state.accessible_keys.iter() {
        task_queue.push(ExplorationTask{next_key: *key.0, explorer: ExplorationState::new(initial_mapping_state.clone(), entrance_position.first().unwrap())});
    }

    let mut min_distance = std::u32::MAX;
    let mut num_obtained_keys = 0;

    while !task_queue.is_empty() {
        // Pick up the next key, prune if a cheaper state already was seen
        while !task_queue.is_empty() {
            let mut task = task_queue.pop().unwrap();
            let explorer = &mut task.explorer;
            let obtained_keys = explorer.obtained_keys.clone();

            if explorer.obtained_keys.len() > num_obtained_keys {
                num_obtained_keys = obtained_keys.len();
                println!("obtained keys: {}", obtained_keys.len());
            }

            explorer.pick_up_key(task.next_key, &map, &obtained_keys);
            explorer.mapping_state.expand(&map, &explorer.obtained_keys);

            if !explorer.mapping_state.accessible_keys.is_empty() {
                explorer.add_to_queue(&mut next_queue);
            }else{
                min_distance = std::cmp::min(min_distance, explorer.distance_travelled);
            }
        }

        // Expand the queued states
        for (_, state) in next_queue.drain().into_iter() {        
            for next_key in state.mapping_state.accessible_keys.iter() {
                let new_task = ExplorationTask{next_key: *next_key.0, explorer: state.clone()};

                task_queue.push(new_task);
            }
        }
    }

    min_distance
}

fn replace(pos: &Vec2u, block: &Map, map: &mut Map) {
    let end_row = pos.r + block.len();
    let end_col = pos.c + block.first().unwrap().len();

    for (block_row, map_row) in (pos.r..end_row).enumerate() {
        for (block_col, map_col) in (pos.c..end_col).enumerate() {
            map[map_row][map_col] = block[block_row][block_col];
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ExplorationTaskMulti {
    next_key: u8,
    next_explorer_idx: usize,
    explorers: Vec<ExplorationState>
}

impl ExplorationTaskMulti {
    fn expand_all(&mut self, map: &Map) {
        let obtained_keys = self.get_obtained_keys();

        for explorer in self.explorers.iter_mut() {
            explorer.mapping_state.expand(map, &obtained_keys);
        }
    }

    fn are_there_keys_left(&self) -> bool {
        for explorer in self.explorers.iter() {
            if explorer.mapping_state.accessible_keys.len() > 0 {
                return true;
            }
        }

        return false;
    }

    fn get_obtained_keys(&self) -> Vec<u8> {
        let mut obtained_keys = Vec::new();

        for explorer in self.explorers.iter() {
            obtained_keys.extend(explorer.obtained_keys.clone());
        }

        obtained_keys
    }

    fn get_current_explorer_positions(&self) -> Vec<Vec2u> {
        let mut positions = Vec::new();

        for explorer in self.explorers.iter() {
            positions.push(explorer.current_position);
        }

        positions
    }

    fn get_total_travelled_distance(&self) -> u32 {
        self.explorers.iter().fold(0u32, |mut sum, state| {sum += state.distance_travelled; sum})
    }


    fn add_to_queue(&self, queue: &mut HashMap<(Vec<u8>, Vec<Vec2u>), Vec<ExplorationState>>) {
        let mut sorted_keys = self.get_obtained_keys();
        sorted_keys.sort();
        let positions = self.get_current_explorer_positions();

        // Check if there already is a state that has obtained the same keys with a lower distance
        match queue.get_mut(&(sorted_keys.clone(), positions.clone())) {
            Some(other) => {
                let rhs_distance = other.iter().fold(0u32, |mut sum, state| {sum += state.distance_travelled; sum});
                let self_distance = self.get_total_travelled_distance();

                if self_distance < rhs_distance {
                    *other = self.explorers.clone();
                }
            },
            None => {
                queue.insert((sorted_keys.clone(), positions.clone()), self.explorers.clone());
            }
        }
    }
}

fn part2(input: String) -> u32 {
    let mut map = build_map(&input);
    let entrance_position = find_entrances(&map).first().unwrap().to_owned();
    let block = build_map(&"@#@\n###\n@#@".to_owned());

    let block_upper_left = Vec2u{r: entrance_position.r - 1, c: entrance_position.c - 1};
    replace(&block_upper_left, &block, &mut map);

    let entrance_positions = find_entrances(&map);
    let mut task_queue = Vec::new();
    let mut next_queue = HashMap::new();


    let mut initial_exploration_states = Vec::new();

    for entrance_position in entrance_positions.into_iter() {
        let mut initial_mapping_state = MappingState::new(entrance_position);
        initial_mapping_state.expand(&map, &Vec::new());
        initial_exploration_states.push(ExplorationState::new(initial_mapping_state, &entrance_position));
    }

    for (idx, initial_exploration_state) in initial_exploration_states.iter().enumerate() {
        for key in initial_exploration_state.mapping_state.accessible_keys.iter() {
            task_queue.push(ExplorationTaskMulti{next_key: *key.0, next_explorer_idx: idx, explorers: initial_exploration_states.clone()});
        }
    }

    let mut min_distance = std::u32::MAX;
    let mut num_obtained_keys: usize = 0;

    while !task_queue.is_empty() {
        // Pick up the next key, prune if a cheaper state already was seen
        while !task_queue.is_empty() {
            let mut task = task_queue.pop().unwrap();
            let obtained_keys = task.get_obtained_keys();

            if obtained_keys.len() > num_obtained_keys {
                num_obtained_keys = obtained_keys.len();
                println!("obtained keys: {}", obtained_keys.len());
            }

            task.explorers[task.next_explorer_idx].pick_up_key(task.next_key, &map, &obtained_keys);
            task.expand_all(&map);


            if task.are_there_keys_left() {
                task.add_to_queue(&mut next_queue);
            }else{
                min_distance = std::cmp::min(min_distance, task.get_total_travelled_distance());
            }
        }

        let mut current_min_distance = std::u32::MAX;

        for task in next_queue.iter() {
            current_min_distance = std::cmp::min(task.1.iter().fold(0u32, |mut sum, state| {sum += state.distance_travelled; sum}), current_min_distance);
        }

        // Expand the queued states
        for (_, explorers) in next_queue.drain().into_iter() {        
            for (explorer_idx, explorer) in explorers.iter().enumerate() {
                for (next_key, _) in explorer.mapping_state.accessible_keys.iter() {
                    let new_task = ExplorationTaskMulti{next_key: *next_key, next_explorer_idx: explorer_idx, explorers: explorers.clone()};

                    // TODO: Eliminate this magic constant
                    if new_task.get_total_travelled_distance() < 2 * current_min_distance {
                        task_queue.push(new_task);
                    }
                }
            }
        }
    }

    min_distance
}

fn main() {
    let input = get_input();

    let steps = part1(input.clone());
    println!("part 1 steps: {}", steps);

    let steps = part2(input);
    println!("part 2 steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test0() {
        let input = "#########\n#b.A.@.a#\n#########".to_owned();

        let steps = part1(input);

        assert_eq!(steps, 8);
    }

    #[test]
    fn part1_test1() {
        let input = "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################".to_owned();

        let steps = part1(input);

        assert_eq!(steps, 86);
    }

    #[test]
    fn part1_test2() {
        let input = "########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################".to_owned();

        let steps = part1(input);

        assert_eq!(steps, 132);
    }

    #[test]
    fn part2_test0() {
        let input = "#######\n#a.#Cd#\n##...##\n##.@.##\n##...##\n#cB#Ab#\n#######".to_owned();

        let steps = part2(input);

        assert_eq!(steps, 8);
    }

    #[test]
    fn part2_test1() {
        let input = "#############\n#DcBa.#.GhKl#\n#.###...#I###\n#e#d#.@.#j#k#\n###C#...###J#\n#fEbA.#.FgHi#\n#############".to_owned();

        let steps = part2(input);

        assert_eq!(steps, 32);
    }
}
