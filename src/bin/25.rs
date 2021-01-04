use std::collections::{VecDeque};
use std::io::{self, Read};

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

enum Command {
    North,
    East,
    South,
    West,
    Take(String),
    Drop(String),
    Inventory
}

impl Command {
    fn to_ascii(&self) -> Vec<i64> {
        match self {
            Command::North => encode_ascii(&"north\n"),
            Command::East => encode_ascii(&"east\n"),
            Command::South => encode_ascii(&"south\n"),
            Command::West => encode_ascii(&"west\n"),
            Command::Take(item) => encode_ascii(&("take ".to_owned() + item + "\n")),
            Command::Drop(item) => encode_ascii(&("drop ".to_owned() + item + "\n")),
            Command::Inventory => encode_ascii(&"inv\n")
        }
    }
}

fn part1(program: &Vec<i64>) -> Result<(),()> {
    let mut vm = IntcodeVM::new(program);

    let mut input_buffer = VecDeque::new();

    loop {
        let mut output_buffer = VecDeque::new();
        
        run(&mut vm, &mut input_buffer, &mut output_buffer);

        let output = decode_ascii(&output_buffer.into_iter().collect());
        println!("{}", output);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input_buffer.append(&mut encode_ascii(&input).into_iter().collect());
    }


    Ok(())
}

intcode_task!(25.txt, part1);

#[cfg(test)]
mod tests {
    use super::*;

}