use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
use std::hash::Hash;

type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Path,
    PortalHalf(char)
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            ' ' => Tile::Empty,
            '.' => Tile::Path,
            'A'..='Z' => Tile::PortalHalf(c),
            _ => panic!("Unknown tile"),
        }
    }
}

fn build_map(input: &String) -> Map {
    let mut map = Map::new();

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut row = Vec::new();

        for char in line.chars() {
                let tile = Tile::from(char);
                row.push(tile);    
        }
        map.push(row);
    }

    map
}

fn get_frame(map: &Map) -> HashSet<Vec2u> {
    let mut queue: VecDeque<Vec2u> = vec![Vec2u{r: 0, c: 0}].into_iter().collect();
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        
        if visited.contains(&current) {
            continue;
        } else {
            visited.insert(current.clone());
        }

        if let Some(tile) = get(map, current.r, current.c) {
            match tile {
                Tile::Empty | Tile::PortalHalf(_) => {
                    for neighbour in current.get_neighbours().into_iter() {
                        queue.push_back(neighbour);
                    }
                }
                _ => ()
            }
        }
    }

    visited
}

fn get_portals(map: &Map) -> HashMap<String, Vec<Vec2u>> {
    let mut portals_coordinates: Vec<(Rc<RefCell<String>>, Vec<Vec2u>)> = Vec::new();
    let mut coordinates_portals = HashMap::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, tile) in row.iter().enumerate() {
            if let Tile::PortalHalf(id) = tile {
                let coord = Vec2u{r: row_index, c: column_index};
                
                if let Some(portal_id) = coordinates_portals.get(&coord) {
                    let portal_index = portals_coordinates.iter().position(|x| Rc::ptr_eq(&x.0, &portal_id)).unwrap();
                    let (portal, coordinates) = portals_coordinates.get_mut(portal_index).unwrap();

                    portal.borrow_mut().push(*id);
                    coordinates.push(coord);
                } else {
                    let portal = Rc::new(RefCell::new(id.to_string()));
                    portals_coordinates.push((portal.clone(), vec![coord]));

                    for neighbor in coord.get_positive_neighbours().iter() {
                        coordinates_portals.insert(neighbor.clone(), portal.clone());
                    }
                }
            }
        }
    }

    let mut portals = HashMap::new();

    for portal in portals_coordinates.iter() {
        for portal_section in portal.1.iter() {
            for neighbour in portal_section.get_neighbours().into_iter() {
                if let Some(Tile::Path) = get(map, neighbour.r, neighbour.c) {
                    portals.entry(portal.0.borrow().clone()).or_insert_with(Vec::new).push(neighbour);
                }
            }
        }
    }

    portals
}

fn get(map: &Map, r: usize, c: usize) -> Option<Tile> {
    if let Some(row) = map.get(r) {
        if let Some(tile) = row.get(c) {
            return Some(*tile);
        }
    }

    None
}

#[derive(Debug)]
struct Destination {
    position: Vec2u,
    level_difference: i32
}

impl Destination {
    fn new(frame: &HashSet<Vec2u>, from_position: &Vec2u, to_position: Vec2u) -> Self {
        let destination;

        if frame.contains(from_position) {
            destination = Destination{position: to_position, level_difference: -1};
        } else {
            destination = Destination{position: to_position, level_difference: 1};
        }

        destination
    }
}

fn get_transfer_table(map: &Map, portals: &HashMap<String, Vec<Vec2u>>) -> HashMap<Vec2u,Destination> {
    let mut transfer_table = HashMap::new();
    let frame = get_frame(map);

    for (_, positions) in portals.iter() {
        if positions.len() != 2 {
            continue;
        }

        let first_position = positions.first().unwrap().clone();
        let last_position = positions.last().unwrap().clone();

        transfer_table.insert(first_position, Destination::new(&frame, &first_position, last_position.clone()));
        transfer_table.insert(last_position, Destination::new(&frame, &last_position, first_position));
    }

    transfer_table
}

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/20.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Vec2u {
    r: usize,
    c: usize,
}

impl Vec2u {
    fn get_positive_neighbours(&self) -> [Vec2u; 2] {
        [Vec2u{r: self.r + 1, c: self.c}, Vec2u{r: self.r, c: self.c + 1}]
    }

    fn get_neighbours(&self) -> Vec<Vec2u> {
        let mut neighbours = Vec::new();

        if self.r > 0 {
            neighbours.push(Vec2u{r: self.r - 1, c: self.c});
        }

        if self.c > 0 {
            neighbours.push(Vec2u{r: self.r, c: self.c - 1});
        }

        neighbours.push(Vec2u{r: self.r + 1, c: self.c});
        neighbours.push(Vec2u{r: self.r, c: self.c + 1});

        neighbours
    }
}

#[derive(Clone)]
struct ExplorationState {
    current_position: Vec2u,
    distance_travelled: u32
}

impl ExplorationState {
    fn new(current_position: Vec2u, distance_travelled: u32) -> ExplorationState {
        ExplorationState{current_position, distance_travelled}
    }
}

fn part1(input: &String) -> u32 {
    
    let map = build_map(input);
    let portals = get_portals(&map);
    let transfer_table = get_transfer_table(&map, &portals);
    let start = *portals.get("AA").unwrap().first().unwrap(); 
    let goal = *portals.get("ZZ").unwrap().first().unwrap();

    let mut queue = vec![ExplorationState::new(start, 0)].into_iter().collect::<VecDeque<_>>();
    let mut visited: HashMap<Vec2u, u32> = HashMap::new();

    let mut min_distance = std::u32::MAX;

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();

        if state.current_position == goal {
            min_distance = std::cmp::min(min_distance, state.distance_travelled);
        }

        if let Some(previous_distance) = visited.get_mut(&state.current_position) {
            if state.distance_travelled < *previous_distance {
                *previous_distance = state.distance_travelled;
            }else{
                continue;
            }
        } else {
            visited.insert(state.current_position, state.distance_travelled);
        }

        for neighbour_position in state.current_position.get_neighbours().into_iter() {
            if let Some(Tile::Path) = get(&map, neighbour_position.r, neighbour_position.c) {
                let next_state = ExplorationState::new(neighbour_position, state.distance_travelled + 1);
                queue.push_back(next_state);
            }
        }

        if let Some(teleport_destination) = transfer_table.get(&state.current_position) {
            let next_state = ExplorationState::new(teleport_destination.position, state.distance_travelled + 1);
            queue.push_back(next_state);
        }
    }

    min_distance
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct LevelExplorationState {
    current_position: Vec2u,
    level: i32,
    distance_travelled: u32
}

impl LevelExplorationState {
    fn new(current_position: Vec2u, level: i32, distance_travelled: u32) -> LevelExplorationState {
        LevelExplorationState{current_position, level, distance_travelled}
    }
}

impl std::cmp::Ord for LevelExplorationState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.level.cmp(&self.level)
    }
}

impl PartialOrd for LevelExplorationState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &String) -> u32 {

    let map = build_map(input);
    let portals = get_portals(&map);
    let transfer_table = get_transfer_table(&map, &portals);
    let start = *portals.get("AA").unwrap().first().unwrap(); 
    let goal = *portals.get("ZZ").unwrap().first().unwrap();

    let mut queue = vec![LevelExplorationState::new(start, 0, 0)].into_iter().collect::<BinaryHeap<_>>();
    let mut visited: HashMap<(Vec2u, i32), u32> = HashMap::new();

    let mut min_distance = std::u32::MAX;

    while !queue.is_empty() {
        let state = queue.pop().unwrap();

        if state.level == 0 && state.current_position == goal {
            min_distance = std::cmp::min(min_distance, state.distance_travelled);
        }

        if state.distance_travelled >= min_distance {
            continue;
        }

        let key = (state.current_position, state.level);

        if let Some(previous_distance) = visited.get_mut(&key) {
            if state.distance_travelled < *previous_distance {
                *previous_distance = state.distance_travelled;
            }else{
                continue;
            }
        } else {
            visited.insert(key, state.distance_travelled);
        }

        for neighbour_position in state.current_position.get_neighbours().into_iter() {
            if let Some(Tile::Path) = get(&map, neighbour_position.r, neighbour_position.c) {
                let next_state = LevelExplorationState::new(neighbour_position, state.level, state.distance_travelled + 1);
                queue.push(next_state);
            }
        }

        if let Some(teleport_destination) = transfer_table.get(&state.current_position) {
            let next_state = LevelExplorationState::new(teleport_destination.position, state.level + teleport_destination.level_difference, state.distance_travelled + 1);
            if next_state.level >= 0 {
                queue.push(next_state);
            }
        }
    }

    /*for (r, row) in map.iter().enumerate() {
        let mut to_print = String::new();
        for (c, col) in row.iter().enumerate() {
            if let Some(destination) = transfer_table.get(&Vec2u{r, c}) {
                match destination.level_difference {
                    1 => to_print += "+",
                    _ => to_print += "-"
                }
            }else{
                match col {
                    Tile::Empty => to_print += " ",
                    Tile::Wall => to_print += "#",
                    Tile::Path => to_print += ".",
                    Tile::PortalHalf(c) => to_print += &c.to_string()
                }
            }
        }
        println!("{}", to_print);
    }*/

    min_distance
}

fn main() {
    let input = get_input();

    let steps = part1(&input);
    println!("part 1 steps: {}", steps);

    let steps = part2(&input);
    println!("part 2 steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input0() -> String {
        r#"
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z     "#.to_owned()
    }

    fn get_input1() -> String {
                  r#"
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P             "#.to_owned()
    }

    #[test]
    fn part1_test0() {
        let input = get_input0();

        let steps = part1(&input);

        assert_eq!(steps, 23);
    }

    #[test]
    fn part1_test1() {
        let input = get_input1();

        let steps = part1(&input);

        assert_eq!(steps, 58);
    }

    #[test]
    fn part2_test0() {
        let input = get_input0();

        let steps = part2(&input);

        assert_eq!(steps, 26);
    }
}
