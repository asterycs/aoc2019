use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Immediate,
    Position,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Mult,
    Add,
    Store,
    Load,
}

fn get_len(op: Op) -> usize {
    match op {
        Op::Mult => 4,
        Op::Add => 4,
        Op::Store => 2,
        Op::Load => 2,
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    modes: [Mode; 3],
    op: Op,
}

impl From<isize> for Instruction {
    fn from(input: isize) -> Instruction {
        let op;

        match input
            .to_string()
            .chars()
            .rev()
            .take(2)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<isize>()
        {
            Ok(1) => op = Op::Add,
            Ok(2) => op = Op::Mult,
            Ok(3) => op = Op::Store,
            Ok(4) => op = Op::Load,
            _ => panic!("Unknown op code"),
        }

        let mut modes = [Mode::Position; 3];
        for (i, c) in input.to_string().chars().rev().skip(2).enumerate() {
            if i > 2 {
                break;
            }

            let d = c.to_digit(10).unwrap();
            if d == 0 {
                modes[i] = Mode::Position;
            } else {
                modes[i] = Mode::Immediate;
            }
        }

        Instruction { modes, op }
    }
}

fn run(mut program: Vec<isize>) -> isize {
    let mut stackptr: usize = 0;

    while stackptr < program.len() {
        if program[stackptr] != 99 {
            let instr = Instruction::from(program[stackptr]);

            let target_addr: usize = match instr.op {
                Op::Add | Op::Mult => program[stackptr + 3] as usize,
                _ => program[stackptr + 1] as usize,
            };

            let a: isize;
            let b: isize;

            match instr.op {
                Op::Add | Op::Mult => {
                    a = if let Mode::Immediate = instr.modes[0] {
                        program[stackptr + 1]
                    } else {
                        program[program[stackptr + 1] as usize]
                    };

                    b = if let Mode::Immediate = instr.modes[1] {
                        program[stackptr + 2]
                    } else {
                        program[program[stackptr + 2] as usize]
                    };
                }
                _ => {
                    a = 0;
                    b = 0;
                }
            }

            match instr.op {
                Op::Add => program[target_addr] = a + b,
                Op::Mult => program[target_addr] = a * b,
                Op::Store => {
                    let read = &mut String::new();
                    io::stdin().read_line(read).expect("malformed input");
                    program[target_addr] = read.trim().parse::<isize>().unwrap();
                }
                Op::Load => println!("{}", program[target_addr]),

                _ => panic!(),
            }

            stackptr += get_len(instr.op);
        } else {
            break;
        }
    }

    return program[0];
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/5_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = &mut input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let res = run(program.clone());
}
