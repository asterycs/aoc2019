use std::collections::VecDeque;
use std::env;
use std::fs;
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
    Read,
    Write,
}

/*trait Op {
    fn execute(&self) -> isize;
    fn get_len(&self) -> usize;
}

struct JumpIfTrue(isize);

impl Op for JumpIfTrue {
    fn execute(&self) {
        if a != 0 {
            true
        }else
    }
}*/

fn get_len(op: Op) -> usize {
    match op {
        Op::JumpIfTrue(_) => 3,
        Op::JumpIfFalse(_) => 3,
        Op::LessThan(_, _) => 4,
        Op::Equals(_, _) => 4,
        Op::Mult(_, _) => 4,
        Op::Add(_, _) => 4,
        Op::Read => 2,
        Op::Write => 2,
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
                        3 => op = Op::Read,
                        4 => op = Op::Write,
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

#[derive(Debug, Clone)]
enum ExecutionStatus {
    Ok,
    Halt,
}

#[derive(Debug, Clone)]
struct ProgramState {
    status: ExecutionStatus,
    stackptr: usize,
    program: Vec<isize>,
}

impl ProgramState {
    fn new(program: Vec<isize>) -> ProgramState {
        ProgramState {
            status: ExecutionStatus::Ok,
            stackptr: 0,
            program,
        }
    }
}

fn step(
    state: &mut ProgramState,
    input_queue: &mut VecDeque<isize>,
    output_queue: &mut VecDeque<isize>,
) {
    let mut hasRead = false;
    let mut hasWritten = false;
    while state.stackptr < state.program.len() {
        if state.program[state.stackptr] != 99 {
            let instr = Instruction::new(&state.program, state.stackptr);

            let mut next = if state.stackptr == instr.target_addr {
                state.stackptr
            } else {
                state.stackptr + get_len(instr.op)
            };

            println!("state.stackptr {}", state.stackptr);
            println!("next: {}", next);
            println!("intr: {:?}", instr.op);

            match instr.op {
                // TODO: Move to method
                Op::Add(a, b) => state.program[instr.target_addr] = a + b,
                Op::Mult(a, b) => state.program[instr.target_addr] = a * b,
                Op::Read => {
                    state.program[instr.target_addr] =
                        input_queue.pop_front().expect("Missing input");
                    hasRead = true;
                }
                Op::Write => {
                    output_queue.push_back(state.program[instr.target_addr]);
                    hasWritten = true;
                }
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
                        state.program[instr.target_addr] = 1;
                    } else {
                        state.program[instr.target_addr] = 0;
                    }
                }
                Op::Equals(a, b) => {
                    if a == b {
                        state.program[instr.target_addr] = 1;
                    } else {
                        state.program[instr.target_addr] = 0;
                    }
                }
            }

            state.stackptr = next;

            if hasRead && hasWritten {
                state.status = ExecutionStatus::Ok;
                return;
            } else {
                continue;
            }
        } else {
            state.status = ExecutionStatus::Halt;
            return;
        }
    }

    state.status = ExecutionStatus::Halt;
    return;
}

#[derive(Debug, Clone)]
struct Permuter {
    x: Vec<isize>,
    c: Vec<usize>,
    i: usize,
}

impl Permuter {
    fn new(init: Vec<isize>) -> Permuter {
        Permuter {
            c: vec![0; init.len()],
            x: init,
            i: 0,
        }
    }

    // https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn next(&self) -> Option<Permuter> {
        let mut x = self.x.clone();
        let mut c = self.c.clone();

        let mut i = self.i;
        while i < x.len() {
            if c[i] < i {
                if i % 2 == 0 {
                    let tmp = x[0];
                    x[0] = x[i];
                    x[i] = tmp;
                } else {
                    let tmp = x[c[i]];
                    x[c[i]] = x[i];
                    x[i] = tmp;
                }

                c[i] += 1;
                i = 0;

                return Some(Permuter { x, c, i });
            } else {
                c[i] = 0;
                i += 1;
            }
        }

        None
    }
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/7_2.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut x = Permuter::new(vec![5, 6, 7, 8, 9]);

    let mut max_sequence = x.clone();
    let mut max_power: isize = 0;

    loop {
        let states = &mut vec![
            ProgramState::new(program.clone()),
            ProgramState::new(program.clone()),
            ProgramState::new(program.clone()),
            ProgramState::new(program.clone()),
            ProgramState::new(program.clone()),
        ];
        let input_queue: &mut VecDeque<isize> = &mut VecDeque::new();
        let output_queue: &mut VecDeque<isize> = &mut vec![0].into_iter().collect();

        let mut i = 0;
        let mut round = 0;
        loop {
            println!("program: {}", i);
            println!("round: {}", round);
            println!("status: {:?}", states[i].status);

            match states[i].status {
                ExecutionStatus::Halt => break,
                _ => (),
            }

            if (output_queue.len() > 0) {
                input_queue.push_front(output_queue.pop_back().unwrap());
            }

            if round == 0 {
                input_queue.push_front(x.x[i]);
            }

            println!("inputs: {:?}", input_queue);
            println!("outputs: {:?}", output_queue);

            step(&mut states[i], &mut *input_queue, &mut *output_queue);

            println!("inputs2: {:?}", input_queue);
            println!("outputs2: {:?}", output_queue);

            println!("status2: {:?}", states[i].status);

            i += 1;

            if i > 4 {
                round += 1;
                i = 0;
            }
        }

        println!("{:?}", input_queue);
        println!("{:?}", output_queue);

        let power = input_queue.pop_back().unwrap();
        if power > max_power {
            max_power = power;
            max_sequence = x.clone();
        }

        if let Some(p) = x.next() {
            x = p;
        } else {
            break;
        }
    }

    println!("maxPower: {}", max_power);
    println!("maxSequence: {:?}", max_sequence);
}
