use std::env;
use std::fs;
use std::path::PathBuf;

fn run(mut program: Vec<usize>) -> usize {
    let mut stackptr: usize = 0;

    while stackptr < program.len() {
        if program[stackptr] != 99 {
            let instr = program[stackptr];
            let a = program[stackptr + 1];
            let b = program[stackptr + 2];
            let c = program[stackptr + 3];

            let res;

            match instr {
                1 => res = program[a] + program[b],
                2 => res = program[a] * program[b],

                _ => panic!(),
            }
            program[c] = res;

            stackptr += 4;
        } else {
            break;
        }
    }

    return program[0];
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/2.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for noun in 0..99 {
        for verb in 0..99 {
            program[1] = noun;
            program[2] = verb;

            let res = run(program.clone());

            if res == 19690720 {
                println!("noun: {} verb: {}", noun, verb);
                println!("res: {}", 100 * noun + verb);
                break;
            }
        }
    }
}
