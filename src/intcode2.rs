use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Immediate,
    Position,
    Relative,
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

trait Instruction {
    fn execute(&self, vm: &mut IntcodeVM, input_buffer: &mut VecDeque<isize>, output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError>;
    fn get_len(&self) -> usize;
}

trait ValueExpression {
    fn execute(&self) -> isize;
    fn get_len(&self) -> usize;
}

struct Put {
    dest: usize,
    op: Box<dyn ValueExpression>,
}

impl Instruction for Put {
    fn execute(&self, vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        drop(vm.memory.insert(self.dest, self.op.execute()));

        if self.dest != vm.instruction_ptr {
            vm.instruction_ptr += self.get_len();
        }

        Ok(())
    }

    fn get_len(&self) -> usize {
        self.op.get_len()
    }
}

struct Add {
    lhs: isize,
    rhs: isize,
}

impl ValueExpression for Add {
    fn execute(&self) -> isize {
        self.lhs + self.rhs
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Mult {
    lhs: isize,
    rhs: isize,
}

impl ValueExpression for Mult {
    fn execute(&self) -> isize {
        self.lhs * self.rhs
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct LessThan {
    lhs: isize,
    rhs: isize,
}

impl ValueExpression for LessThan {
    fn execute(&self) -> isize {
        match self.lhs < self.rhs {
            true => 1,
            false => 0,
        }
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Equals {
    lhs: isize,
    rhs: isize,
}

impl ValueExpression for Equals {
    fn execute(&self) -> isize {
        match self.lhs == self.rhs {
            true => 1,
            false => 0,
        }
    }

    fn get_len(&self) -> usize {
        4
    }
}

struct Input {
    dest: usize,
}

impl Instruction for Input {
    fn execute(&self, vm: &mut IntcodeVM, input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        if input_buffer.is_empty() {
            Err(ExecutionError::EmptyInputBuffer)
        } else {
            drop(
                vm.memory
                    .insert(self.dest, input_buffer.pop_front().unwrap()),
            );

            if self.dest != vm.instruction_ptr {
                vm.instruction_ptr += self.get_len();
            }

            Ok(())
        }
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct Output {
    val: isize,
}

impl Instruction for Output {
    fn execute(&self, vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        vm.instruction_ptr += self.get_len();

        output_buffer.push_back(self.val);
        Ok(())
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct JumpIfTrue {
    operand: isize,
    dest: usize,
}

impl Instruction for JumpIfTrue {
    fn execute(&self, vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        match self.operand {
            0 => vm.instruction_ptr += self.get_len(),
            _ => vm.instruction_ptr = self.dest,
        }

        Ok(())
    }

    fn get_len(&self) -> usize {
        3
    }
}

struct JumpIfFalse {
    operand: isize,
    dest: usize,
}

impl Instruction for JumpIfFalse {
    fn execute(&self, vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        match self.operand {
            0 => vm.instruction_ptr = self.dest,
            _ => vm.instruction_ptr += self.get_len(),
        }

        Ok(())
    }

    fn get_len(&self) -> usize {
        3
    }
}

struct UpdateRelativeBase {
    offset: isize,
}

impl Instruction for UpdateRelativeBase {
    fn execute(&self, vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        vm.instruction_ptr += self.get_len();
        vm.relative_base += self.offset;
        Ok(())
    }

    fn get_len(&self) -> usize {
        2
    }
}

struct Halt {}

impl Instruction for Halt {
    fn execute(&self, _vm: &mut IntcodeVM, _input_buffer: &mut VecDeque<isize>, _output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
        Err(ExecutionError::Halted)
    }

    fn get_len(&self) -> usize {
        1
    }
}

impl Mode {
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
            Mode::Position => {
                let t = program[&instruction_ptr] as usize;
                t
            }
            Mode::Relative => {
                let t = (program[&instruction_ptr] + relative_base as isize) as usize;
                t
            }
        }
    }
}

impl dyn Instruction {
    fn next(vm: &mut IntcodeVM) -> Box<dyn Instruction> {
        let p = &vm.memory;
        let ip = vm.instruction_ptr;
        let modes = Mode::get_next_3(vm);
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
                1 | 2 | 7 | 8 => {
                    let a = modes[0].get_value(p, ip + 1, vm.relative_base);
                    let b = modes[1].get_value(p, ip + 2, vm.relative_base);
                    let dest = modes[2].get_addr(p, ip + 3, vm.relative_base);

                    match opcode {
                        1 => {
                            instruction = Box::new(Put {
                                dest: dest,
                                op: Box::new(Add { lhs: a, rhs: b }),
                            })
                        }
                        2 => {
                            instruction = Box::new(Put {
                                dest: dest,
                                op: Box::new(Mult { lhs: a, rhs: b }),
                            })
                        }
                        7 => {
                            instruction = Box::new(Put {
                                dest: dest,
                                op: Box::new(LessThan { lhs: a, rhs: b }),
                            })
                        }
                        8 => {
                            instruction = Box::new(Put {
                                dest: dest,
                                op: Box::new(Equals { lhs: a, rhs: b }),
                            })
                        }
                        _ => unreachable!(),
                    };
                }
                3 => {
                    let dest = modes[0].get_addr(p, ip + 1, vm.relative_base);
                    instruction = Box::new(Input { dest });
                }
                4 => {
                    let val = modes[0].get_value(p, ip + 1, vm.relative_base);
                    instruction = Box::new(Output { val });
                }
                5 | 6 => {
                    let operand = modes[0].get_value(p, ip + 1, vm.relative_base);
                    let dest = modes[1].get_value(p, ip + 2, vm.relative_base) as usize;

                    match opcode {
                        5 => {
                            instruction = Box::new(JumpIfTrue {
                                dest,
                                operand,
                            })
                        }
                        6 => {
                            instruction = Box::new(JumpIfFalse {
                                dest,
                                operand,
                            })
                        }
                        _ => unreachable!(),
                    };
                }
                9 => {
                    let a = modes[0].get_value(p, ip + 1, vm.relative_base);
                    instruction = Box::new(UpdateRelativeBase { offset: a });
                }
                99 => {
                    instruction = Box::new(Halt {});
                }
                _ => panic!("Unknown opcode: {}", opcode),
            },
            _ => panic!("Invalid opcode"),
        }

        instruction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionError {
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

pub fn run(vm: &mut IntcodeVM, input_buffer: &mut VecDeque<isize>, output_buffer: &mut VecDeque<isize>) -> Result<(), ExecutionError> {
    loop {
        let instr = Instruction::next(vm);
        instr.execute(vm, input_buffer, output_buffer)?;
    }
}
