use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/5.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut input_buffer = vec![1].into_iter().collect();
    let mut output_buffer = VecDeque::new();
    let mut vm = IntcodeVM::new(&program);

    run(&mut vm, &mut input_buffer, &mut output_buffer);

    println!("Part 1: {:?}", output_buffer);

    let mut input_buffer = vec![5].into_iter().collect();
    let mut output_buffer = VecDeque::new();
    let mut vm = IntcodeVM::new(&program);

    run(&mut vm, &mut input_buffer, &mut output_buffer);

    println!("Part 2: {:?}", output_buffer);
}
