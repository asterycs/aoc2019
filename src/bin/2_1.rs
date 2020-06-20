use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let filename = &mut PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    filename.push("inputs/2_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    program[1] = 12;
    program[2] = 2;

    let mut i: usize = 0;

    while i < program.len() {
        if program[i] != 99 {
            let instr = program[i];
            let a = program[i + 1];
            let b = program[i + 2];
            let c = program[i + 3];

            let res = match instr {
                1 => program[a] + program[b],
                2 => program[a] * program[b],

                _ => panic!(),
            };
            
            program[c] = res;

            i += 4;
        } else {
            break;
        }
    }

    println!("{:?}", program);
}
