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
    JumpIfTrue(isize, usize),
    JumpIfFalse(isize, usize),
    LessThan(isize, isize, usize),
    Equals(isize, isize, usize),
    Mult(isize, isize, usize),
    Add(isize, isize, usize),
    Store(usize),
    Load(usize),
}

fn get_len(op: Op) -> usize {
    match op {
        Op::JumpIfTrue(_, _) => 3,
        Op::JumpIfFalse(_, _) => 3,
        Op::LessThan(_, _, _) => 4,
        Op::Equals(_, _, _) => 4,
        Op::Mult(_, _, _) => 4,
        Op::Add(_, _, _) => 4,
        Op::Store(_) => 2,
        Op::Load(_) => 2,
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    modes: [Mode; 3],
    op: Op,
}

impl Instruction {
    fn new(program: &Vec<isize>, stackptr: usize) -> Instruction {
        let mut modes = [Mode::Position; 3];
        for (i, c) in program[stackptr]
            .to_string()
            .chars()
            .rev()
            .skip(2)
            .enumerate()
        {
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

        let op;
        match program[stackptr]
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
            Ok(opcode) => match opcode {
                // idea for next time: Parse the operation beforehand into an enum OpType {One(opcode), Three(opcode)}
                1 | 2 | 7 | 8 => {
                    let a = match modes[0] {
                        Mode::Immediate => program[stackptr + 1],
                        Mode::Position => program[program[stackptr + 1] as usize],
                    };
                    let b = match modes[1] {
                        Mode::Immediate => program[stackptr + 2],
                        Mode::Position => program[program[stackptr + 2] as usize],
                    };

                    let c = program[stackptr + 3] as usize;

                    match opcode {
                        1 => op = Op::Add(a, b, c),
                        2 => op = Op::Mult(a, b, c),
                        7 => op = Op::LessThan(a, b, c),
                        8 => op = Op::Equals(a, b, c),
                        _ => unreachable!(),
                    }
                }
                3 | 4 => {
                    let c = program[stackptr + 1] as usize;
                    match opcode {
                        3 => op = Op::Store(c),
                        4 => op = Op::Load(c),
                        _ => unreachable!(),
                    };
                }
                5 | 6 => {
                    let a = match modes[0] {
                        Mode::Immediate => program[stackptr + 1],
                        Mode::Position => program[program[stackptr + 1] as usize],
                    };
                    let c = match modes[1] {
                        Mode::Immediate => program[stackptr + 2],
                        Mode::Position => program[program[stackptr + 2] as usize],
                    } as usize;

                    match opcode {
                        5 => op = Op::JumpIfTrue(a, c),
                        6 => op = Op::JumpIfFalse(a, c),
                        _ => unreachable!(),
                    };
                }
                _ => panic!("Unknown opcode: {}", opcode),
            },
            _ => panic!("Invalid opcode"),
        }

        Instruction { modes, op }
    }
}

fn run(mut program: Vec<isize>) -> isize {
    let mut stackptr: usize = 0;

    while stackptr < program.len() {
        if program[stackptr] != 99 {
            let instr = Instruction::new(&program, stackptr);

            let target_addr;
            match instr.op {
                // TODO: Move c to separate field in Instruction
                Op::Add(_, _, c) => target_addr = c,
                Op::Mult(_, _, c) => target_addr = c,
                Op::Store(c) => target_addr = c,
                Op::Load(c) => target_addr = c,
                Op::JumpIfTrue(_, c) => target_addr = c,
                Op::JumpIfFalse(_, c) => target_addr = c,
                Op::LessThan(_, _, c) => target_addr = c,
                Op::Equals(_, _, c) => target_addr = c,
            };

            let mut next = if stackptr == target_addr {
                stackptr
            } else {
                stackptr + get_len(instr.op)
            };

            match instr.op {
                // TODO: Move to method
                Op::Add(a, b, c) => program[c] = a + b,
                Op::Mult(a, b, c) => program[c] = a * b,
                Op::Store(c) => {
                    let read = &mut String::new();
                    io::stdin().read_line(read).expect("malformed input");
                    program[c] = read.trim().parse::<isize>().unwrap();
                }
                Op::Load(c) => println!("{}", program[c]),
                Op::JumpIfTrue(a, c) => match a {
                    0 => (),
                    _ => next = c,
                },
                Op::JumpIfFalse(a, c) => match a {
                    0 => next = c,
                    _ => (),
                },
                Op::LessThan(a, b, c) => {
                    if a < b {
                        program[c] = 1;
                    } else {
                        program[c] = 0;
                    }
                }
                Op::Equals(a, b, c) => {
                    if a == b {
                        program[c] = 1;
                    } else {
                        program[c] = 0;
                    }
                }
            }

            stackptr = next;
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
