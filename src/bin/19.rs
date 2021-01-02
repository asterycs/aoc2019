use std::{char::from_digit, collections::HashMap};
use std::collections::VecDeque;

use common::*;
use intcode::*;

const SIZE: u32 = 100;

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
    fn new(coordinates: &VecDeque<Vec2u>, output: &VecDeque<i64>) -> Map {
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

fn part1(program: &Vec<i64>) -> i64 {
    let coordinates = &mut VecDeque::new();
    let output_queue = &mut VecDeque::new();

    for x in 0..50 {
        for y in 0..50 {
            let mut vm = IntcodeVM::new(&program);

            let input_queue = &mut VecDeque::new();

            input_queue.push_back(x as i64);
            input_queue.push_back(y as i64);

            run(&mut vm, input_queue, output_queue);

            coordinates.push_back(Vec2u{x, y});
        }
    }

    let map = Map::new(coordinates, output_queue);
    draw_view(&map);

    output_queue.iter().sum()
}

fn has_traction(coordinate: &Vec2u, program: &Vec<i64>) -> bool {
    let input_queue = &mut VecDeque::new();
    let output_queue = &mut VecDeque::new();

    input_queue.push_back(coordinate.x as i64);
    input_queue.push_back(coordinate.y as i64);

    let mut vm = IntcodeVM::new(program);
    run(&mut vm, input_queue, output_queue);

    output_queue.pop_front().unwrap() == 1
}

fn fits_in_x(program: &Vec<i64>, upper_right_corner: &Vec2u) -> bool {
    let mut runner = *upper_right_corner;
    for _x in 0..SIZE {
        runner.x -= 1;

        if !has_traction(&runner, program) {
            return false;
        }
    }

    true
}

fn fits_in_y(program: &Vec<i64>, upper_left_corner: &Vec2u) -> bool {
    let mut runner = *upper_left_corner;
    for _y in 0..SIZE-1 {
        runner.y += 1;
        if !has_traction(&runner, program) {
            return false;
        }
    }

    true
}

fn part2(program: &Vec<i64>) -> u32 {
    let mut upper_right_corner = Vec2u{x: 4, y: 3};

    loop {
        if fits_in_x(&program, &upper_right_corner) && fits_in_y(&program, &Vec2u{x: upper_right_corner.x - SIZE + 1, y: upper_right_corner.y}) {
            println!("found it!  Upper right: {:?}", upper_right_corner);
            break;
        }else{
            upper_right_corner.y += 1;

            loop {
                let upper_right_corner_candidate = Vec2u{x: upper_right_corner.x + 1, y: upper_right_corner.y};

                if !has_traction(&upper_right_corner_candidate, &program) {
                    break;
                }

                upper_right_corner.x += 1;
            }
        }
    }

    (upper_right_corner.x - SIZE + 1) * 10000 + upper_right_corner.y
}

intcode_task!(19.txt, part1, part2);