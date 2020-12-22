use std::{char::from_digit, collections::HashMap};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

fn get_input() -> String {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/19.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    input
}

#[derive(Debug, Hash, Eq, Copy, Clone)]
struct Vec2u {
    x: u32,
    y: u32,
}

impl PartialEq for Vec2u {
    fn eq(&self, r: &Vec2u) -> bool {
        self.x == r.x && self.y == r.y
    }
}

#[derive(Clone)]
struct Map {
    size: Vec2u,
    data: HashMap<Vec2u, u8>
}

impl Map {
    fn new(coordinates: &VecDeque<Vec2u>, output: &VecDeque<isize>) -> Map {
        let mut data = HashMap::new();
        let mut size = Vec2u{x: 0, y: 0};

        for (coordinate, value) in coordinates.iter().zip(output.iter()) {
            data.insert(coordinate.clone(), value.clone() as u8);
            size.x = std::cmp::max(size.x, coordinate.x);
            size.y = std::cmp::max(size.y, coordinate.y);
        }

        Map {
            size,
            data
        }
    }
}


fn draw_view(map: &Map) {
    let mut to_draw = String::new();

    for y in 0..map.size.y {
        for x in 0..map.size.x {
            to_draw.push(from_digit(*map.data.get(&Vec2u { x, y }).unwrap() as u32, 10).unwrap());
        }
        to_draw.push('\n');
    }

    print!("{}", to_draw);
}

fn main() {
    let input = get_input();

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let coordinates = &mut VecDeque::new();
    let output_queue = &mut VecDeque::new();

    for x in 0..50 {
        for y in 0..50 {
            let mut vm = IntcodeVM::new(program);

            let input_queue = &mut VecDeque::new();

            input_queue.push_back(x as isize);
            input_queue.push_back(y as isize);

            run(&mut vm, input_queue, output_queue);

            coordinates.push_back(Vec2u{x, y});
        }
    }

    let map = Map::new(coordinates, output_queue);
    draw_view(&map);

    let sum: isize = output_queue.iter().sum();

    println!("part1: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

}
