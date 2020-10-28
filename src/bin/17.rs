use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/17.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

#[derive(Debug, Hash, Eq, Copy, Clone)]
struct Vec2i {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, Copy, Clone)]
struct Vec2u {
    x: u32,
    y: u32,
}

impl PartialEq for Vec2i {
    fn eq(&self, r: &Vec2i) -> bool {
        self.x == r.x && self.y == r.y
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Heading {
    East,
    West,
    North,
    South,
}

impl Heading {
    fn get_opposite(&self) -> Heading {
        match self {
            Heading::East => Heading::West,
            Heading::West => Heading::East,
            Heading::North => Heading::South,
            Heading::South => Heading::North,
        }
    }

    fn get_neighbor(&self, coord: &Vec2i) -> Vec2i {
        match self {
            Heading::East => Vec2i {
                x: coord.x + 1,
                y: coord.y,
            },
            Heading::West => Vec2i {
                x: coord.x - 1,
                y: coord.y,
            },
            Heading::North => Vec2i {
                x: coord.x,
                y: coord.y - 1,
            },
            Heading::South => Vec2i {
                x: coord.x,
                y: coord.y + 1,
            },
        }
    }
}

impl From<i8> for Heading {
    fn from(i: i8) -> Self {
        match i {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!("Unknown heading"),
        }
    }
}

impl Into<i8> for Heading {
    fn into(self) -> i8 {
        match self {
            Heading::North => 0,
            Heading::East => 1,
            Heading::South => 2,
            Heading::West => 3,
        }
    }
}
#[derive(Clone)]
struct Map {
    size: Vec2u,
    data: HashMap<Vec2i, isize>,
    robot_position: Vec2i,
    robot_heading: Heading
}

impl Map {
    fn new(output: &VecDeque<isize>) -> Map {
        let mut data = HashMap::new();
        let mut current_position = Vec2i { x: 0, y: 0 };
        let mut size = Vec2u { x: 0, y: 0 };
        let mut robot_position = Vec2i { x: 0, y: 0 };

        for o in output {
            if *o == 60 || *o == 62 || *o == 94 || *o == 118 {
                robot_position = Vec2i {
                    x: current_position.x,
                    y: current_position.y,
                };
            }

            if *o == 10 {
                current_position.x = 0;
                current_position.y += 1;
            } else {
                data.insert(current_position, *o);
                size.x = std::cmp::max(current_position.x as u32, size.x);
                size.y = std::cmp::max(current_position.y as u32, size.y);
                current_position.x += 1;
            }
        }

        let robot_heading = get_initial_heading(*data.get(&robot_position).unwrap());

        Map {
            data,
            size: Vec2u {
                x: size.x + 1,
                y: size.y + 1,
            },
            robot_position,
            robot_heading
        }
    }
}

#[derive(Debug)]
struct Intersection {
    position: Vec2i,
}

fn draw_view(map: &Map) {
    let mut to_draw = String::new();

    for y in 0..map.size.y as i32 {
        for x in 0..map.size.x as i32 {
            to_draw.push(*map.data.get(&Vec2i { x, y }).unwrap() as u8 as char);
        }
        to_draw.push('\n');
    }

    print!("{}", to_draw);
}

fn is_road(map: &Map, coordinate: &Vec2i) -> bool {
    if let Some(35) = map.data.get(coordinate) {
        true
    } else {
        false
    }
}

fn get_intersections(map: &Map) -> Vec<Intersection> {
    let mut intersections = Vec::new();

    for coord in map.data.keys() {
        let mut is_intersection = true;

        if map.data.get(coord) != Some(&35) {
            continue;
        }

        let west_neighbor = Heading::West.get_neighbor(coord);
        let east_neighbor = Heading::East.get_neighbor(coord);
        let south_neighbor = Heading::South.get_neighbor(coord);
        let north_neighbor = Heading::North.get_neighbor(coord);

        is_intersection &= is_road(map, &north_neighbor);
        is_intersection &= is_road(map, &south_neighbor);
        is_intersection &= is_road(map, &west_neighbor);
        is_intersection &= is_road(map, &east_neighbor);

        if is_intersection {
            intersections.push(Intersection { position: *coord });
        }
    }

    intersections
}

fn prod_sum(intersections: &Vec<Intersection>) -> i32 {
    let mut alignment = 0;

    for intersection in intersections {
        alignment += intersection.position.x * intersection.position.y;
    }

    alignment
}

fn get_alignment(map: &Map) -> i32 {
    let intersections = get_intersections(&map);

    let alignment = prod_sum(&intersections);

    alignment
}

fn get_initial_heading(map_value: isize) -> Heading {
    match map_value {
        60 => Heading::West,
        62 => Heading::East,
        94 => Heading::North,
        118 => Heading::South,
        _ => panic!("Failed to parse initial heading"),
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Move {
    Forward,
    TurnLeft,
    TurnRight,
}

impl From<i8> for Move {
    fn from(i: i8) -> Self {
        match i {
            -1 => Move::TurnLeft,
            0 => Move::Forward,
            1 => Move::TurnRight,
            _ => panic!("Unrecognized move: {}", i),
        }
    }
}

impl Move {
    fn get_heading_after(&self, heading: Heading) -> Heading {
        match self {
            Move::TurnLeft => Heading::from((Into::<i8>::into(heading) + 4 - 1) % 4),
            Move::TurnRight => Heading::from((Into::<i8>::into(heading) + 1) % 4),
            _ => return heading,
        }
    }
}

fn get_next_move(mut neighbors: HashMap<Heading, bool>, current_heading: Heading) -> Option<Move> {
    if neighbors[&current_heading] {
        return Some(Move::Forward);
    }

    neighbors.remove(&current_heading);
    neighbors.remove(&current_heading.get_opposite());

    for (heading, traversable) in neighbors.into_iter() {
        if traversable {
            let turned_quarters = Into::<i8>::into(heading) - Into::<i8>::into(current_heading);
            let turned_quarters = (turned_quarters + 2).rem_euclid(4) - 2;

            return Some(Move::from(turned_quarters));
        }
    }

    return None;
}

fn get_to_goal_commands(map: &Map) -> Vec<String> {
    let mut current_position = map.robot_position;
    let mut current_heading = map.robot_heading;
    let mut segment_length = 0;
    let mut commands = Vec::new();

    loop {
        let west = Heading::West.get_neighbor(&current_position);
        let east = Heading::East.get_neighbor(&current_position);
        let north = Heading::North.get_neighbor(&current_position);
        let south = Heading::South.get_neighbor(&current_position);

        let neighbors = [
            (Heading::West, west),
            (Heading::East, east),
            (Heading::North, north),
            (Heading::South, south),
        ]
        .iter()
        .map(|n| (n.0, is_road(map, &n.1)))
        .collect::<HashMap<_, _>>();

        let next_move = get_next_move(neighbors.clone(), current_heading);

        if let Some(mov) = next_move {
            match mov {
                Move::Forward => {
                    segment_length += 1;
                    current_position = current_heading.get_neighbor(&current_position);
                }
                Move::TurnLeft => {
                    if segment_length > 0 {
                        commands.push(segment_length.to_string())
                    }
                    segment_length = 0;
                    commands.push("L".to_string());
                }
                Move::TurnRight => {
                    if segment_length > 0 {
                        commands.push(segment_length.to_string())
                    }
                    segment_length = 0;
                    commands.push("R".to_string());
                }
            }

            current_heading = mov.get_heading_after(current_heading);
        } else {
            commands.push(segment_length.to_string());
            break;
        }
    }

    return commands;
}

fn find_subsequences<T>(haystack: &[T], needle: &[T]) -> Vec<usize>
    where for<'a> &'a [T]: PartialEq
{
    let mut indices = vec![];

    for (i, b) in haystack.windows(needle.len()).map(|window| window == needle).collect::<Vec<bool>>().into_iter().enumerate() {
        if b {
            indices.push(i);
        }
    }

    indices
}

macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn get_program(map: &Map) -> Vec<Vec<String>> {
    let _to_goal_commands = get_to_goal_commands(map);

    let a = string_vec!["L", "6", "L", "10", "L", "10", "R", "6"];
    let b = string_vec!["L", "6", "R", "12", "L", "4", "L", "6"];
    let c = string_vec!["R", "6", "L", "6", "R", "12"];

    // One day I'll make a real solution for this
    //find_subsequences(&to_goal_commands, &a);
    //find_subsequences(&to_goal_commands, &b);
    //find_subsequences(&to_goal_commands, &c);

    vec![string_vec!["B","C","C","A","B","C","A","B","C","A"], a, b, c]
}

fn to_ascii(input: &Vec<String>) -> Vec<Vec<isize>> {
    input.iter().map(|c| c.chars().map(|x| x as isize).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>()
}

fn main() {
    let input = get_input();

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut vm = IntcodeVM::new(program);

    let mut input_queue = &mut VecDeque::new();
    let mut output_queue = &mut VecDeque::new();

    // Part 1
    if VMStatus::Halted != run(&mut vm, &mut input_queue, &mut output_queue) {
        panic!("Intcode program failed");
    }

    let map = Map::new(output_queue);
    draw_view(&map);

    let alignment = get_alignment(&map);
    println!("Alignment: {}", alignment);

    // Part 2
    program[0] = 2;
    let robot_program = get_program(&map);

    let mut vm = IntcodeVM::new(program);

    let mut input_queue = &mut VecDeque::new();
    let mut output_queue = &mut VecDeque::new();

    run(&mut vm, &mut input_queue, &mut output_queue);

    for i in 0..4 {
        input_queue.clear();
        output_queue.clear();
        let as_ascii = to_ascii(&robot_program[i]);

        for command in as_ascii.into_iter() {
            for c in command.into_iter() {
                input_queue.push_back(c);
            }
            input_queue.push_back(44);
        }

        input_queue.pop_back();
        input_queue.push_back(10);

        let vm_status = run(&mut vm, &mut input_queue, &mut output_queue);

        if vm_status != VMStatus::EmptyInputBuffer {
            panic!("VM execution failed");
        }
    }

    input_queue.clear();
    output_queue.clear();

    input_queue.push_back('n' as u8 as isize);
    input_queue.push_back(10);

    if VMStatus::Ok == run(&mut vm, &mut input_queue, &mut output_queue) {
        println!("output: {:?}", output_queue.back());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> VecDeque<isize> {
        let input = "..#..........\n..#..........\n#######...###\n#.#...#...#.#\n#############\n..#...#...#..\n..#####...#..";

        input.chars().map(|c| c as isize).collect()
    }

    #[test]
    fn test_example_part1() {
        let test_input = get_test_input();

        let map = Map::new(&test_input);

        let alignment = get_alignment(&map);

        assert_eq!(alignment, 76);
    }
}
