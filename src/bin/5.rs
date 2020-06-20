use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use intcode::*;

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/5_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut input_queue: VecDeque<isize> = vec![1].into_iter().collect();
    let mut output_queue: VecDeque<isize> = VecDeque::new();
    let mut state = ProgramState::new(&program);

    run(&mut state, &mut input_queue, &mut output_queue);

    println!("Part 1: {:?}", output_queue);

    let mut input_queue: VecDeque<isize> = vec![5].into_iter().collect();
    let mut output_queue: VecDeque<isize> = VecDeque::new();
    let mut state = ProgramState::new(&program);

    run(&mut state, &mut input_queue, &mut output_queue);

    println!("Part 2: {:?}", output_queue);
}
