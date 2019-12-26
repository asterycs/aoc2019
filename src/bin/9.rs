use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use aoc::intcode::*;

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/9_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let input_queue: &mut VecDeque<isize> = &mut vec![1].into_iter().collect();
    let output_queue: &mut VecDeque<isize> = &mut VecDeque::new();

    let mut state = ProgramState::new(&program);
    run(&mut state, &mut *input_queue, &mut *output_queue);

    println!("Part 1: {:?}", output_queue);

    let input_queue: &mut VecDeque<isize> = &mut vec![2].into_iter().collect();
    let output_queue: &mut VecDeque<isize> = &mut VecDeque::new();

    let mut state = ProgramState::new(&program);
    run(&mut state, &mut *input_queue, &mut *output_queue);

    println!("Part 2: {:?}", output_queue);
}
