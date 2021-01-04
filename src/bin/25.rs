use std::collections::{VecDeque};

use common::*;
use intcode::*;

fn part1(program: &Vec<i64>) -> Result<(),()> {
    let mut vm = IntcodeVM::new(program);

    let mut input_buffer = VecDeque::new();

    loop {
        let mut output_buffer = VecDeque::new();
        
        run(&mut vm, &mut input_buffer, &mut output_buffer);

        let output = decode_ascii(&output_buffer.into_iter().collect());
        println!("{}", output);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input_buffer.append(&mut encode_ascii(&input).into_iter().collect());
    }
}

intcode_task!(25.txt, part1);