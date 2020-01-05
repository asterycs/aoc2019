use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

use aoc::intcode::*;

fn draw_screen(output: &Vec<isize>) {

    let mut row_idx = 0;
    let mut line = Vec::new();

    let mut block_cntr = 0;

    let mut score = 0;

    for c in output.chunks(3) {
        let x = c[0];
        let y = c[1];
        let t = c[2];

        let sign;

        if x == -1 && y == 0 {
            score = t;
            continue;
        }

        match t {
            0 => sign = ' ',
            1 => sign = 'w',
            2 => {sign = '#'; block_cntr += 1;}
            3 => sign = '-',
            4 => sign = 'o',
            _ => panic!("Unknown sign"),
        }

        if y == row_idx {
        line.push(sign);
        }else{
            println!("{}", line.iter().collect::<String>());
            line.clear();
            line.push(sign);
            row_idx = y;
        }
    }

    println!("Score: {}", score);

    //println!("blocks: {}", block_cntr);
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/13_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();


    // Part 1
    let input_queue = &mut VecDeque::new();
    let output_queue = &mut VecDeque::new();

    let mut state = ProgramState::new(program);

    run(&mut state, &mut *input_queue, &mut *output_queue);

    // Hmm, unnecesessary copy, yes...
    draw_screen(&output_queue.clone().into_iter().collect::<Vec<_>>());

    // Part 2
    program[0] = 2;
    let mut state = ProgramState::new(program);

    loop {
        let input_queue = &mut vec![0].into_iter().collect();
        let output_queue = &mut VecDeque::new();

        run(&mut state, &mut *input_queue, &mut *output_queue);
        draw_screen(&output_queue.clone().into_iter().collect::<Vec<_>>());
    }
}
