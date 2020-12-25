use std::env;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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

fn get_transfer_table(portals: &HashMap<String, Vec<Vec2u>>) -> HashMap<Vec2u,Vec2u> {
    let mut transfer_table = HashMap::new();

    for (_, positions) in portals.iter() {
        if positions.len() != 2 {
            continue;
        }

        transfer_table.insert(positions.first().unwrap().clone(), positions.last().unwrap().clone());
        transfer_table.insert(positions.last().unwrap().clone(), positions.first().unwrap().clone());
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

fn part1(input: String) -> u32 {
    
    let map = build_map(&input);
    let portals = get_portals(&map);
    let transfer_table = get_transfer_table(&portals);
    let start = *portals.get("AA").unwrap().first().unwrap(); 
    let goal = *portals.get("ZZ").unwrap().first().unwrap();

    let mut queue = vec![ExplorationState::new(start, 0)].into_iter().collect::<VecDeque<_>>();
    let mut visited: HashMap<Vec2u, ExplorationState> = HashMap::new();

    let mut min_distance = std::u32::MAX;

    while !queue.is_empty() {
        let state = queue.pop_back().unwrap();

        if state.current_position == goal {
            min_distance = std::cmp::min(min_distance, state.distance_travelled);
        }

        if let Some(previously_visited_state) = visited.get_mut(&state.current_position) {
            if state.distance_travelled < previously_visited_state.distance_travelled {
                *previously_visited_state = state.clone();
            }else{
                continue;
            }
        } else {
            visited.insert(state.current_position, state.clone());
        }

        for neighbour_position in state.current_position.get_neighbours().into_iter() {
            if let Some(Tile::Path) = get(&map, neighbour_position.r, neighbour_position.c) {
                let next_state = ExplorationState::new(neighbour_position, state.distance_travelled + 1);
                queue.push_back(next_state);
            }
        }

        if let Some(teleport_position) = transfer_table.get(&state.current_position) {
            let next_state = ExplorationState::new(*teleport_position, state.distance_travelled + 1);
            queue.push_back(next_state);
        }
    }

    min_distance
}

fn main() {
    let input = get_input();

    let steps = part1(input.clone());
    println!("part 1 steps: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test0() {
        let input = r#"
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
             Z           
            "#.to_owned();

        let steps = part1(input);

        assert_eq!(steps, 23);
    }

    #[test]
    fn part1_test1() {
        let input = r#"
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
           U   P   P                                        
            "#.to_owned();

        let steps = part1(input);

        assert_eq!(steps, 58);
    }
}
