use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Tank,
    Robot,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Command {
    MoveNorth = 0,
    MoveWest = 1,
    MoveSouth = 2,
    MoveEast = 3,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CommandResult {
    HitWall = 0,
    Ok = 1,
    FoundTank = 2,
}

impl From<isize> for Command {
    fn from(x: isize) -> Self {
        match x {
            0 => Command::MoveNorth,
            1 => Command::MoveWest,
            2 => Command::MoveSouth,
            3 => Command::MoveEast,
            _ => panic!("Unknown command"),
        }
    }
}

impl From<isize> for CommandResult {
    fn from(x: isize) -> Self {
        match x {
            0 => CommandResult::HitWall,
            1 => CommandResult::Ok,
            2 => CommandResult::FoundTank,
            _ => panic!("Unknown output"),
        }
    }
}

#[derive(Debug, Hash, Eq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl PartialEq for Coord {
    fn eq(&self, r: &Coord) -> bool {
        self.x == r.x && self.y == r.y
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, r: Coord) -> Self {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

impl From<isize> for Tile {
    fn from(x: isize) -> Self {
        match x {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Tank,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Into<&str> for &Tile {
    fn into(self) -> &'static str {
        match self {
            Tile::Wall => "#",
            Tile::Empty => ".",
            Tile::Tank => "X",
            Tile::Robot => "D",
        }
    }
}

#[derive(Debug)]
struct SearchState {
    map: HashMap<Coord, Tile>,
    robot_pos: Coord,
    tank_pos: Option<Coord>,
}

fn get_internal_command_code(command: Command) -> isize {
    match command {
        Command::MoveNorth => 1,
        Command::MoveSouth => 2,
        Command::MoveWest => 3,
        Command::MoveEast => 4,
    }
}

impl SearchState {
    fn new() -> SearchState {
        let mut map = HashMap::new();
        map.insert(Coord { x: 0, y: 0 }, Tile::Empty);

        SearchState {
            map: map,
            robot_pos: Coord { x: 0, y: 0 },
            tank_pos: None,
        }
    }

    fn advance(&mut self, vm: &mut IntcodeVM, command: Command) -> CommandResult {
        let intcode_command = get_internal_command_code(command);

        let mut input_queue = vec![intcode_command as isize]
            .into_iter()
            .collect::<VecDeque<isize>>();
        let mut output_queue = &mut VecDeque::new();

        run(vm, &mut input_queue, &mut output_queue);

        let response = CommandResult::from(*output_queue.back().unwrap());

        let next_pos = match command {
            Command::MoveNorth => self.robot_pos + Coord { x: 0, y: 1 },
            Command::MoveSouth => self.robot_pos + Coord { x: 0, y: -1 },
            Command::MoveWest => self.robot_pos + Coord { x: -1, y: 0 },
            Command::MoveEast => self.robot_pos + Coord { x: 1, y: 0 },
        };

        match response {
            CommandResult::HitWall => {
                self.map.insert(next_pos, Tile::Wall);
            }
            CommandResult::Ok => {
                self.map.insert(next_pos.clone(), Tile::Empty);
                self.robot_pos = next_pos;
            }
            CommandResult::FoundTank => {
                self.map.insert(next_pos.clone(), Tile::Tank);
                self.robot_pos = next_pos;
                self.tank_pos = Some(next_pos);
            }
        }

        response
    }
}

fn draw(state: &SearchState) {
    let max_x = state.map.keys().max_by_key(|c| c.x).unwrap().x;
    let max_y = state.map.keys().max_by_key(|c| c.y).unwrap().y;
    let min_x = state.map.keys().min_by_key(|c| c.x).unwrap().x;
    let min_y = state.map.keys().min_by_key(|c| c.y).unwrap().y;

    let mut to_draw = String::new();

    let mut map_tmp = state.map.clone();
    map_tmp.insert(state.robot_pos, Tile::Robot);

    for y in (min_y..max_y + 1).rev() {
        for x in (min_x..max_x + 1).rev() {
            if let Some(c) = state.map.get(&Coord { x, y }) {
                to_draw += c.into();
            } else {
                to_draw += " ";
            }
        }

        to_draw += "\n";
    }

    print!("{}", to_draw);
}

#[derive(Debug, Clone, Copy)]
struct Node {
    coord: Coord,
    distance: usize,
}

fn bfs(map: &HashMap<Coord, Tile>, start: Coord, mut target: HashSet<Coord>) -> usize {
    let queue = &mut vec![Node {
        coord: start,
        distance: 0,
    }]
    .into_iter()
    .collect::<VecDeque<_>>();

    let finished = &mut HashSet::new();

    let mut min_dist = 0;

    while !queue.is_empty() && !target.is_empty() {
        let current = queue.pop_front().unwrap();
        finished.insert(current.coord);

        let north = Coord {
            x: current.coord.x,
            y: current.coord.y + 1,
        };
        let west = Coord {
            x: current.coord.x - 1,
            y: current.coord.y,
        };
        let south = Coord {
            x: current.coord.x,
            y: current.coord.y - 1,
        };
        let east = Coord {
            x: current.coord.x + 1,
            y: current.coord.y,
        };

        let neighbors = vec![north, west, south, east];

        for neighbor in neighbors {
            if finished.contains(&neighbor) {
                continue;
            }

            if let Some(t) = map.get(&neighbor) {
                match t {
                    Tile::Empty => queue.push_back(Node {
                        coord: neighbor,
                        distance: current.distance + 1,
                    }),
                    _ => (),
                }

                min_dist = current.distance + 1;
                drop(target.remove(&neighbor));
            }
        }
    }

    min_dist
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/15.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut vm = IntcodeVM::new(program);
    let mut search_state = SearchState::new();
    let mut command = Command::MoveNorth;
    let origin = Coord { x: 0, y: 0 };

    loop {
        let result = search_state.advance(&mut vm, command);

        draw(&search_state);

        println!("Robot at: {:?}", search_state.robot_pos);
        println!("heading: {:?}", command);

        if result == CommandResult::HitWall {
            command = Command::from((command as isize + 1) % 4);
        } else {
            command = Command::from((command as isize + 4 - 1) % 4);
        }

        if search_state.robot_pos == origin && command == Command::MoveNorth {
            break;
        }
    }

    let tank_location = vec![search_state.tank_pos.unwrap()].into_iter().collect();
    let tank_distance = bfs(&search_state.map, Coord { x: 0, y: 0 }, tank_location);

    println!("Tank distance: {}", tank_distance);

    let target_location = search_state // all empty tiles
        .map
        .clone()
        .into_iter()
        .filter(|&(_k, v)| v == Tile::Empty)
        .collect::<HashMap<Coord, Tile>>()
        .keys()
        .cloned()
        .collect();
    let min_time = bfs(
        &search_state.map,
        search_state.tank_pos.unwrap(),
        target_location,
    );

    println!("Minimum time: {}", min_time);
}
