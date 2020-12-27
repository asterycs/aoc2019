use std::collections::HashMap;
use std::collections::VecDeque;

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

impl Mode {
    fn get_next_3(vm: &IntcodeVM) -> [Mode; 3] {
        let mut modes = [Mode::Position; 3];
        for (i, c) in vm.memory[&vm.instruction_ptr]
            .to_string()
            .chars()
            .rev()
            .skip(2)
            .enumerate()
        {
            if i > 2 {
                panic!("Unsupported number of operands");
            }

            modes[i] = Mode::from(c.to_digit(10).unwrap());
        }

        modes
    }

    fn get_value(
        &self,
        program: &HashMap<usize, isize>,
        instruction_ptr: usize,
        relative_base: isize,
    ) -> isize {
        match self {
            Mode::Immediate => program[&instruction_ptr],
            Mode::Position => {
                let t = program[&instruction_ptr] as usize;
                *program.get(&t).unwrap_or(&0)
            }
            Mode::Relative => {
                let t = (program[&instruction_ptr] + relative_base as isize) as usize;
                *program.get(&t).unwrap_or(&0)
            }
        }
    }

    fn get_addr(
        &self,
        program: &HashMap<usize, isize>,
        instruction_ptr: usize,
        relative_base: isize,
    ) -> usize {
        match self {
            Mode::Immediate => panic!("Invalid mode!"),
            Mode::Position => program[&instruction_ptr] as usize,
            Mode::Relative => (program[&instruction_ptr] + relative_base as isize) as usize,
        }
    }
}

trait Instruction {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        input_buffer: &mut VecDeque<isize>,
        output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus;
    fn get_len(&self) -> usize;
}

struct Add {
    lhs: isize,
    rhs: isize,
    dest: usize,
}

impl Add {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let lhs = modes[0].get_value(p, ip + 1, vm.relative_base);
        let rhs = modes[1].get_value(p, ip + 2, vm.relative_base);
        let dest = modes[2].get_addr(p, ip + 3, vm.relative_base);

        Add { lhs, rhs, dest }
    }
}

impl Instruction for Add {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        vm.memory.insert(self.dest, self.lhs + self.rhs);

        if self.dest != vm.instruction_ptr {
            vm.instruction_ptr += self.get_len();
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Mult {
    lhs: isize,
    rhs: isize,
    dest: usize,
}

impl Mult {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let lhs = modes[0].get_value(p, ip + 1, vm.relative_base);
        let rhs = modes[1].get_value(p, ip + 2, vm.relative_base);
        let dest = modes[2].get_addr(p, ip + 3, vm.relative_base);

        Mult { lhs, rhs, dest }
    }
}

impl Instruction for Mult {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        vm.memory.insert(self.dest, self.lhs * self.rhs);

        if self.dest != vm.instruction_ptr {
            vm.instruction_ptr += self.get_len();
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct LessThan {
    lhs: isize,
    rhs: isize,
    dest: usize,
}

impl LessThan {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let lhs = modes[0].get_value(p, ip + 1, vm.relative_base);
        let rhs = modes[1].get_value(p, ip + 2, vm.relative_base);
        let dest = modes[2].get_addr(p, ip + 3, vm.relative_base);

        LessThan { lhs, rhs, dest }
    }
}

impl Instruction for LessThan {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        let res = match self.lhs < self.rhs {
            true => 1,
            false => 0,
        };

        vm.memory.insert(self.dest, res);

        if self.dest != vm.instruction_ptr {
            vm.instruction_ptr += self.get_len();
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Equals {
    lhs: isize,
    rhs: isize,
    dest: usize,
}

impl Equals {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let lhs = modes[0].get_value(p, ip + 1, vm.relative_base);
        let rhs = modes[1].get_value(p, ip + 2, vm.relative_base);
        let dest = modes[2].get_addr(p, ip + 3, vm.relative_base);

        Equals { lhs, rhs, dest }
    }
}

impl Instruction for Equals {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        let res = match self.lhs == self.rhs {
            true => 1,
            false => 0,
        };

        vm.memory.insert(self.dest, res);

        if self.dest != vm.instruction_ptr {
            vm.instruction_ptr += self.get_len();
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Input {
    dest: usize,
}

impl Input {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let dest = modes[0].get_addr(p, ip + 1, vm.relative_base);

        Input { dest }
    }
}

impl Instruction for Input {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        if input_buffer.is_empty() {
            VMStatus::EmptyInputBuffer
        } else {
            vm.memory.insert(self.dest, input_buffer.pop_front().unwrap());

            if self.dest != vm.instruction_ptr {
                vm.instruction_ptr += self.get_len();
            }

            VMStatus::Ok
        }
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct Output {
    val: isize,
}

impl Output {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let val = modes[0].get_value(p, ip + 1, vm.relative_base);

        Output { val }
    }
}

impl Instruction for Output {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        vm.instruction_ptr += self.get_len();

        output_buffer.push_back(self.val);

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct JumpIfTrue {
    operand: isize,
    dest: usize,
}

impl JumpIfTrue {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let operand = modes[0].get_value(p, ip + 1, vm.relative_base);
        let dest = modes[1].get_value(p, ip + 2, vm.relative_base) as usize;

        JumpIfTrue { operand, dest }
    }
}

impl Instruction for JumpIfTrue {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        match self.operand {
            0 => vm.instruction_ptr += self.get_len(),
            _ => vm.instruction_ptr = self.dest,
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        3
    }
}

struct JumpIfFalse {
    operand: isize,
    dest: usize,
}

impl JumpIfFalse {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let operand = modes[0].get_value(p, ip + 1, vm.relative_base);
        let dest = modes[1].get_value(p, ip + 2, vm.relative_base) as usize;

        JumpIfFalse { operand, dest }
    }
}

impl Instruction for JumpIfFalse {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        match self.operand {
            0 => vm.instruction_ptr = self.dest,
            _ => vm.instruction_ptr += self.get_len(),
        }

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        3
    }
}

struct UpdateRelativeBase {
    offset: isize,
}

impl UpdateRelativeBase {
    fn new(vm: &IntcodeVM) -> Self {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;

        let modes = Mode::get_next_3(vm);
        let offset = modes[0].get_value(p, ip + 1, vm.relative_base);

        UpdateRelativeBase { offset }
    }
}

impl Instruction for UpdateRelativeBase {
    fn execute(
        &self,
        vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        vm.instruction_ptr += self.get_len();
        vm.relative_base += self.offset;

        VMStatus::Ok
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct Halt {}

impl Halt {
    fn new(_vm: &IntcodeVM) -> Self {
        Halt {}
    }
}

impl Instruction for Halt {
    fn execute(
        &self,
        _vm: &mut IntcodeVM,
        _input_buffer: &mut VecDeque<isize>,
        _output_buffer: &mut VecDeque<isize>,
    ) -> VMStatus {
        VMStatus::Halted
    }

    fn get_len(&self) -> usize {
        1
    }
}

impl dyn Instruction {
    fn next(vm: &mut IntcodeVM) -> Box<dyn Instruction> {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;
        let instruction: Box<dyn Instruction>;

        match p[&ip]
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
                1 => instruction = Box::new(Add::new(vm)),
                2 => instruction = Box::new(Mult::new(vm)),
                3 => instruction = Box::new(Input::new(vm)),
                4 => instruction = Box::new(Output::new(vm)),
                5 => instruction = Box::new(JumpIfTrue::new(vm)),
                6 => instruction = Box::new(JumpIfFalse::new(vm)),
                7 => instruction = Box::new(LessThan::new(vm)),
                8 => instruction = Box::new(Equals::new(vm)),
                9 => instruction = Box::new(UpdateRelativeBase::new(vm)),
                99 => instruction = Box::new(Halt::new(vm)),
                _ => panic!("Unknown opcode: {}", opcode),
            },
            _ => panic!("Invalid opcode"),
        }

        instruction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VMStatus {
    Ok,
    EmptyInputBuffer,
    Halted,
}

#[derive(Debug, Clone)]
pub struct IntcodeVM {
    instruction_ptr: usize,
    relative_base: isize,
    pub memory: HashMap<usize, isize>,
}

impl IntcodeVM {
    pub fn new(program: &Vec<isize>) -> IntcodeVM {
        IntcodeVM {
            instruction_ptr: 0,
            relative_base: 0,
            memory: program
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.clone()))
                .collect(),
        }
    }
}

pub fn run(
    vm: &mut IntcodeVM,
    input_buffer: &mut VecDeque<isize>,
    output_buffer: &mut VecDeque<isize>,
) -> VMStatus {
    loop {
        let instr = Instruction::next(vm);
        let status = instr.execute(vm, input_buffer, output_buffer);
        if let VMStatus::Ok = status {
            continue;
        }

        return status;
    }
}
