use std::collections::HashMap;
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
    fn get_value(&self, program: &HashMap<usize, isize>, stackptr: usize, relative_base: isize) -> isize {
        match self {
            Mode::Immediate => program[&stackptr],
            Mode::Position => {
                let t = program[&stackptr] as usize;
                *program.get(&t).unwrap_or(&0)
            }
            Mode::Relative => {
                let t = (program[&stackptr] + relative_base as isize) as usize;
                *program.get(&t).unwrap_or(&0)
            }
        }
    }

    fn get_addr(&self, program: &HashMap<usize, isize>, stackptr: usize, relative_base: isize) -> usize {
        match self {
            Mode::Immediate => panic!("Invalid mode!"),
            Mode::Position => {
                let t = program[&stackptr] as usize;
                t
            }
            Mode::Relative => {
                let t = (program[&stackptr] + relative_base as isize) as usize;
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
    fn next(state: &ProgramState) -> Instruction {
        let p = &state.memory;
        let sp = state.stackptr;

        let mut modes = [Mode::Position; 3];
        for (i, c) in p[&sp].to_string().chars().rev().skip(2).enumerate() {
            if i > 2 {
                break;
            }

            modes[i] = Mode::from(c.to_digit(10).unwrap());
        }

        let op;
        match p[&sp]
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
    memory: HashMap<usize, isize>,
}

impl ProgramState {
    fn new(program: &Vec<isize>) -> ProgramState {
        ProgramState {
            status: ExecutionStatus::Waiting,
            stackptr: 0,
            relative_base: 0,
            memory: program.iter().enumerate().map(|(i,x)| (i,x.clone())).collect(),
        }
    }
}

fn run(
    state: &mut ProgramState,
    input_queue: &mut VecDeque<isize>,
    output_queue: &mut VecDeque<isize>,
) {
    while state.stackptr < state.memory.len() {
        let instr = Instruction::next(state);

        let mut next = state.stackptr + instr.op.get_len();
        if let Target::Addr(t, _) = instr.op {
            if t == state.stackptr {
                next = state.stackptr;
            }
        };

        match instr.op {
            Target::Addr(addr, op) => match op {
                Op::Add(a, b) => drop(state.memory.insert(addr, a + b)),
                Op::Mult(a, b) => drop(state.memory.insert(addr, a * b)),
                Op::LessThan(a, b) => drop(state.memory.insert(addr, if a < b { 1 } else { 0 })),
                Op::Equals(a, b) => drop(state.memory.insert(addr, if a == b { 1 } else { 0 })),
                Op::Input => {
                    if input_queue.is_empty() {
                        state.status = ExecutionStatus::Waiting;
                        return;
                    }

                    drop(state.memory.insert(addr, input_queue.pop_front().expect("Missing input")))
                },
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

struct RobotState {
    x: isize,
    y: isize,
    dir: isize,
}

impl RobotState {
    fn turn(&mut self, dir: isize) {
        let dir = if dir == 1 { 1 }else{ -1 };

        self.dir += dir;
        self.dir = self.dir.rem_euclid(4);
    }

    fn move_forward(&mut self) {
        let dx;
        let dy;

        match self.dir {
            0 => { dx =  0; dy =  1; },
            1 => { dx =  1; dy =  0; },
            2 => { dx =  0; dy = -1; },
            3 => { dx = -1; dy =  0; },
            _ => unreachable!("Invalid move dir"),
        }

        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl From<isize> for Color {
    fn from(i: isize) -> Color {
        match i {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Unknown color"),
        }
    }
}

impl Into<isize> for &Color {
    fn into(self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

fn draw(map: &HashMap<(isize,isize), Color>) {
    let mut min_x = std::isize::MAX;
    let mut min_y = std::isize::MAX;

    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;

    for (k,_) in map {
        min_x = std::cmp::min(min_x, k.0);
        min_y = std::cmp::min(min_y, k.1);

        max_x = std::cmp::max(max_x, k.0);
        max_y = std::cmp::max(max_y, k.1);
    }
    
    for r in (min_y..max_y+1).rev() {
        for c in min_x..max_x+1 {
            let col = map.get(&(c,r)).unwrap_or(&Color::Black);
            let sign;

            match col {
                Color::Black => sign = '.',
                Color::White => sign = '#',
            }

            print!("{}", sign);
        }
        println!();
    }


}

fn run_painter(init_tile_color: isize, program: &Vec<isize>) -> HashMap<(isize, isize), Color> {
    let input_queue: &mut VecDeque<isize> = &mut vec![init_tile_color].into_iter().collect();
    let output_queue: &mut VecDeque<isize> = &mut VecDeque::new();

    let mut program_state = ProgramState::new(&program);
    let mut robot_state = RobotState { x: 0, y: 0, dir: 0 };
    let mut hull: HashMap<(isize, isize), Color> = HashMap::new();

    loop {
        run(&mut program_state, &mut *input_queue, &mut *output_queue);

        let color = Color::from(output_queue.pop_front().unwrap());
        let direction = output_queue.pop_front().unwrap();

        hull.insert((robot_state.x, robot_state.y), color);

        robot_state.turn(direction);
        robot_state.move_forward();

        input_queue.push_back(hull.get(&(robot_state.x, robot_state.y)).unwrap_or(&Color::Black).into());

        match program_state.status {
            ExecutionStatus::Halt | ExecutionStatus::Error => break,
            _ => continue,
        }
    }

    hull
}

fn main() {
    let filename = &mut PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    filename.push("inputs/11_1.txt");

    println!("Reading {}", filename.display());

    let input = fs::read_to_string(filename).expect("Unable to open file");

    let program = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    
    let hull = run_painter(0, &program);
    println!("printed panels: {}", hull.len());

    let hull = run_painter(1, &program);
    draw(&hull);
}
