use std::{char::from_digit, collections::HashMap};
use std::collections::VecDeque;

use common::*;
use intcode::*;

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

fn part1(program: Vec<isize>) -> Result<(),()> {
    let mut output_queue = VecDeque::new();
    let mut vm = IntcodeVM::new(&program);

    let springscript =
r#"OR D J
NOT C T
AND T J
NOT A T
OR T J
WALK
"#.to_owned();

    let mut input_queue: VecDeque<_> = encode_ascii(&springscript).into_iter().collect();

    run(&mut vm, &mut input_queue, &mut output_queue);

    println!("output (ascii): {}", decode_ascii(&output_queue.clone().into_iter().collect()));
    println!("output (raw): {:?}", output_queue.back());
    
    Ok(())
}

intcode_task!(21.txt, part1);
