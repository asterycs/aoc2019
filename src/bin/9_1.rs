use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl From<u32> for Mode {
    fn from(m: u32) -> Mode {
        match m {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    UpdateRelativeBase(isize),
    JumpIfTrue(isize),
    JumpIfFalse(isize),
    LessThan(isize, isize),
    Equals(isize, isize),
    Mult(isize, isize),
    Add(isize, isize),
    Input,
    Output,
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Target {
    Addr(usize, Op),
    Value(isize, Op),
    Jump(usize, Op),
    UpdateState(Op),
}

impl Op {
    fn get_len(&self) -> usize {
        match self {
            Op::UpdateRelativeBase(_) => 2,
            Op::JumpIfTrue(_) => 3,
            Op::JumpIfFalse(_) => 3,
            Op::LessThan(_, _) => 4,
            Op::Equals(_, _) => 4,
            Op::Mult(_, _) => 4,
            Op::Add(_, _) => 4,
            Op::Input => 2,
            Op::Output => 2,
            Op::Halt => 2,
        }
    }
}

impl Target {
    fn get_len(&self) -> usize {
        match self {
            Target::Addr(_, op) => op.get_len(),
            Target::Value(_, op) => op.get_len(),
            Target::Jump(_, op) => op.get_len(),
            Target::UpdateState(op) => op.get_len(),
        }
    }
}

impl Mode {
    fn get_value(&self, program: &mut Vec<isize>, stackptr: usize, relative_base: isize) -> isize {
        match self {
            Mode::Immediate => program[stackptr],
            Mode::Position => {
                let t = program[stackptr] as usize;
                if t > program.len() {
                    program.resize(t + 1, 0);
                }
                program[t]
            }
            Mode::Relative => {
                let t = (program[stackptr] + relative_base as isize) as usize;
                if t >= program.len() {
                    program.resize(t + 1, 0);
                }
                program[t]
            }
        }
    }

    fn get_addr(&self, program: &mut Vec<isize>, stackptr: usize, relative_base: isize) -> usize {
        match self {
            Mode::Immediate => panic!("Invalid mode!"),
            Mode::Position => {
                let t = program[stackptr] as usize;
                if t > program.len() {
                    program.resize(t + 1, 0);
                }
                t
            }
            Mode::Relative => {
                let t = (program[stackptr] + relative_base as isize) as usize;
                if t >= program.len() {
                    program.resize(t + 1, 0);
                }
                t
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    modes: [Mode; 3],
    op: Target,
}

impl Instruction {
    fn new(state: &mut ProgramState) -> Instruction {
        let p = &mut state.program;
        let sp = state.stackptr;

        let mut modes = [Mode::Position; 3];
        for (i, c) in p[sp].to_string().chars().rev().skip(2).enumerate() {
            if i > 2 {
                break;
            }

            modes[i] = Mode::from(c.to_digit(10).unwrap());
        }

        let op;
        match p[sp]
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
                1 | 2 | 7 | 8 => {
                    let a = modes[0].get_value(p, sp + 1, state.relative_base);
                    let b = modes[1].get_value(p, sp + 2, state.relative_base);
                    let t = modes[2].get_addr(p, sp + 3, state.relative_base);

                    match opcode {
                        1 => op = Target::Addr(t, Op::Add(a, b)),
                        2 => op = Target::Addr(t, Op::Mult(a, b)),
                        7 => op = Target::Addr(t, Op::LessThan(a, b)),
                        8 => op = Target::Addr(t, Op::Equals(a, b)),
                        _ => unreachable!(),
                    }
                }
                3 => {
                    let t = modes[0].get_addr(p, sp + 1, state.relative_base);
                    op = Target::Addr(t as usize, Op::Input);
                }
                4 => {
                    let v = modes[0].get_value(p, sp + 1, state.relative_base);
                    op = Target::Value(v, Op::Output);
                }
                5 | 6 => {
                    let a = modes[0].get_value(p, sp + 1, state.relative_base);
                    let n = modes[1].get_value(p, sp + 2, state.relative_base) as usize;

                    match opcode {
                        5 => op = Target::Jump(n, Op::JumpIfTrue(a)),
                        6 => op = Target::Jump(n, Op::JumpIfFalse(a)),
                        _ => unreachable!(),
                    };
                }
                9 => {
                    let a = modes[0].get_value(p, sp + 1, state.relative_base);
                    op = Target::UpdateState(Op::UpdateRelativeBase(a));
                }
                99 => {
                    op = Target::UpdateState(Op::Halt);
                }
                _ => panic!("Unknown opcode: {}", opcode),
            },
            _ => panic!("Invalid opcode"),
        }

        Instruction { modes, op }
    }
}

#[derive(Debug, Clone)]
enum ExecutionStatus {
    Waiting,
    Halt,
    Error,
}

#[derive(Debug, Clone)]
struct ProgramState {
    status: ExecutionStatus,
    stackptr: usize,
    relative_base: isize,
    program: Vec<isize>,
}

impl ProgramState {
    fn new(program: Vec<isize>) -> ProgramState {
        ProgramState {
            status: ExecutionStatus::Waiting,
            stackptr: 0,
            relative_base: 0,
            program,
        }
    }
}

fn run(
    state: &mut ProgramState,
    input_queue: &mut VecDeque<isize>,
    output_queue: &mut VecDeque<isize>,
) {
    while state.stackptr < state.program.len() {
        let instr = Instruction::new(state);

        let mut next = state.stackptr + instr.op.get_len();
        if let Target::Addr(t, _) = instr.op {
            if t == state.stackptr {
                next = state.stackptr;
            }
        };

        /*println!("state.stackptr {}", state.stackptr);
        println!("next: {}", next);
        println!("op: {:?}", instr.op);
        println!("relative_base: {:?}", state.relative_base);
        println!("modes: {:?}", instr.modes);*/

        match instr.op {
            Target::Addr(addr, op) => match op {
                Op::Add(a, b) => state.program[addr] = a + b,
                Op::Mult(a, b) => state.program[addr] = a * b,
                Op::LessThan(a, b) => state.program[addr] = if a < b { 1 } else { 0 },
                Op::Equals(a, b) => state.program[addr] = if a == b { 1 } else { 0 },
                Op::Input => state.program[addr] = input_queue.pop_front().expect("Missing input"),
                _ => unreachable!(),
            },
            Target::Value(val, op) => match op {
                Op::Output => output_queue.push_back(val),
                _ => unreachable!(),
            },
            Target::Jump(n, op) => match op {
                Op::JumpIfTrue(a) => match a {
                    0 => (),
                    _ => next = n,
                },
                Op::JumpIfFalse(a) => match a {
                    0 => next = n,
                    _ => (),
                },
                _ => unreachable!(),
            },
            Target::UpdateState(op) => match op {
                Op::UpdateRelativeBase(a) => {
                    state.relative_base = state.relative_base + a;
                }
                Op::Halt => {
                    state.status = ExecutionStatus::Halt;
                    return;
                }
                _ => unreachable!(),
            },
        }

        state.stackptr = next;
    }

    state.status = ExecutionStatus::Error;
    return;
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/9_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let input_queue: &mut VecDeque<isize> = &mut vec![2].into_iter().collect();
    let output_queue: &mut VecDeque<isize> = &mut VecDeque::new();

    let mut state = ProgramState::new(program);

    run(&mut state, &mut *input_queue, &mut *output_queue);

    println!("{:?}", input_queue);
    println!("{:?}", output_queue);
}
