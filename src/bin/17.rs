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

fn get_test_input() -> VecDeque<isize> {
    let input = "..#..........\n..#..........\n#######...###\n#.#...#...#.#\n#############\n..#...#...#..\n..#####...#..";

    input.chars().map(|c| c as isize).collect()
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

struct Map {
    size: Vec2u,
    data: HashMap<Vec2i, isize>,
}

impl Map {
    fn new(output: &VecDeque<isize>) -> Map {
        let mut data = HashMap::new();
        let mut current_pos = Vec2i { x: 0, y: 0 };
        let mut size = Vec2u { x: 0, y: 0 };

        for o in output {
            if *o == 10 {
                current_pos.x = 0;
                current_pos.y += 1;
            } else {
                data.insert(current_pos, *o);
                size.x = std::cmp::max(current_pos.x as u32, size.x);
                size.y = std::cmp::max(current_pos.y as u32, size.y);
                current_pos.x += 1;
            }
        }

        Map {
            data,
            size: Vec2u {
                x: size.x + 1,
                y: size.y + 1,
            },
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

fn get_intersections(map: &Map) -> Vec<Intersection> {
    let mut intersections = Vec::new();

    for coord in map.data.keys() {
        let mut is_intersection = true;

        if map.data.get(coord) != Some(&35) {
            continue;
        }

        let west_neighbor = Vec2i {
            x: coord.x - 1,
            y: coord.y,
        };

        if let Some(35) = map.data.get(&west_neighbor) {
            is_intersection = true && is_intersection;
        } else {
            is_intersection = false;
        }

        let east_neighbor = Vec2i {
            x: coord.x + 1,
            y: coord.y,
        };

        if let Some(35) = map.data.get(&east_neighbor) {
            is_intersection = true && is_intersection;
        } else {
            is_intersection = false;
        }

        let south_neighbor = Vec2i {
            x: coord.x,
            y: coord.y - 1,
        };

        if let Some(35) = map.data.get(&south_neighbor) {
            is_intersection = true && is_intersection;
        } else {
            is_intersection = false;
        }

        let north_neighbor = Vec2i {
            x: coord.x,
            y: coord.y + 1,
        };

        if let Some(35) = map.data.get(&north_neighbor) {
            is_intersection = true && is_intersection;
        } else {
            is_intersection = false;
        }

        if is_intersection {
            intersections.push(Intersection { position: *coord });
        }
    }

    intersections
}

fn get_alignment(intersections: &Vec<Intersection>) -> i32 {
    let mut alignment = 0;

    for intersection in intersections {
        alignment += intersection.position.x * intersection.position.y;
    }

    alignment
}

fn part_1(map_input: &VecDeque<isize>) -> i32 {
    let map = Map::new(map_input);

    draw_view(&map);

    let intersections = get_intersections(&map);

    let alignment = get_alignment(&intersections);

    alignment
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

    if let VMStatus::Halted = run(&mut vm, &mut input_queue, &mut output_queue) {
        let alignment = part_1(output_queue);

        println!("Alignment: {}", alignment);
    } else {
        panic!("Intcode program failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_input = get_test_input();

        let alignment = part_1(&test_input);

        assert_eq!(alignment, 76);
    }
}
