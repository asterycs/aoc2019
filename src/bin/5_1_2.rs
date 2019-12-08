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
    JumpIfTrue(isize),
    JumpIfFalse(isize),
    LessThan(isize, isize),
    Equals(isize, isize),
    Mult(isize, isize),
    Add(isize, isize),
    Store,
    Load,
}

fn get_len(op: Op) -> usize {
    match op {
        Op::JumpIfTrue(_) => 3,
        Op::JumpIfFalse(_) => 3,
        Op::LessThan(_, _) => 4,
        Op::Equals(_, _) => 4,
        Op::Mult(_, _) => 4,
        Op::Add(_, _) => 4,
        Op::Store => 2,
        Op::Load => 2,
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    modes: [Mode; 3],
    op: Op,
    target_addr: usize,
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
        let target_addr;
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

                    target_addr = program[stackptr + 3] as usize;

                    match opcode {
                        1 => op = Op::Add(a, b),
                        2 => op = Op::Mult(a, b),
                        7 => op = Op::LessThan(a, b),
                        8 => op = Op::Equals(a, b),
                        _ => unreachable!(),
                    }
                }
                3 | 4 => {
                    target_addr = program[stackptr + 1] as usize;
                    match opcode {
                        3 => op = Op::Store,
                        4 => op = Op::Load,
                        _ => unreachable!(),
                    };
                }
                5 | 6 => {
                    let a = match modes[0] {
                        Mode::Immediate => program[stackptr + 1],
                        Mode::Position => program[program[stackptr + 1] as usize],
                    };
                    target_addr = match modes[1] {
                        Mode::Immediate => program[stackptr + 2],
                        Mode::Position => program[program[stackptr + 2] as usize],
                    } as usize;

                    match opcode {
                        5 => op = Op::JumpIfTrue(a),
                        6 => op = Op::JumpIfFalse(a),
                        _ => unreachable!(),
                    };
                }
                _ => panic!("Unknown opcode: {}", opcode),
            },
            _ => panic!("Invalid opcode"),
        }

        Instruction {
            modes,
            op,
            target_addr,
        }
    }
}

fn run(mut program: Vec<isize>) -> isize {
    let mut stackptr: usize = 0;

    while stackptr < program.len() {
        if program[stackptr] != 99 {
            let instr = Instruction::new(&program, stackptr);

            let mut next = if stackptr == instr.target_addr {
                stackptr
            } else {
                stackptr + get_len(instr.op)
            };

            match instr.op {
                // TODO: Move to method
                Op::Add(a, b) => program[instr.target_addr] = a + b,
                Op::Mult(a, b) => program[instr.target_addr] = a * b,
                Op::Store => {
                    let read = &mut String::new();
                    io::stdin().read_line(read).expect("malformed input");
                    program[instr.target_addr] = read.trim().parse::<isize>().unwrap();
                }
                Op::Load => println!("{}", program[instr.target_addr]),
                Op::JumpIfTrue(a) => match a {
                    0 => (),
                    _ => next = instr.target_addr,
                },
                Op::JumpIfFalse(a) => match a {
                    0 => next = instr.target_addr,
                    _ => (),
                },
                Op::LessThan(a, b) => {
                    if a < b {
                        program[instr.target_addr] = 1;
                    } else {
                        program[instr.target_addr] = 0;
                    }
                }
                Op::Equals(a, b) => {
                    if a == b {
                        program[instr.target_addr] = 1;
                    } else {
                        program[instr.target_addr] = 0;
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
