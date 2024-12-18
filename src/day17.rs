use log::debug;
use num_derive::FromPrimitive;
use regex::Regex;

use crate::prelude::*;

struct VM<'a, O> {
    registers: [isize; 3],
    program: &'a [usize],
    instruction_pointer: usize,
    output: O,
}

impl<'a, O: FnMut(isize) -> bool> VM<'a, O> {
    fn new(registers: [isize; 3], program: &'a [usize], output: O) -> Self {
        Self {
            registers,
            program,
            instruction_pointer: 0,
            output,
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let result = match self.instruction() {
            Instruction::Adv => {
                self.registers[0] =
                    self.registers[0] / (2isize.pow(self.combo_operand().try_into().unwrap()));
                InstructionResult::Continue
            }
            Instruction::Bxl => {
                self.registers[1] = self.registers[1] ^ self.literal_operand();
                InstructionResult::Continue
            }
            Instruction::Bst => {
                self.registers[1] = self.combo_operand() % 8;
                InstructionResult::Continue
            }
            Instruction::Jnz => {
                if self.registers[0] != 0 {
                    InstructionResult::Jump(self.literal_operand().try_into().unwrap())
                } else {
                    InstructionResult::Continue
                }
            }
            Instruction::Bxc => {
                self.registers[1] = self.registers[1] ^ self.registers[2];
                InstructionResult::Continue
            }
            Instruction::Out => {
                let op = self.combo_operand() % 8;
                if (self.output)(op) {
                    InstructionResult::Continue
                } else {
                    InstructionResult::Halt
                }
            }
            Instruction::Bdv => {
                self.registers[1] =
                    self.registers[0] / (2isize.pow(self.combo_operand().try_into().unwrap()));
                InstructionResult::Continue
            }
            Instruction::Cdv => {
                self.registers[2] =
                    self.registers[0] / (2isize.pow(self.combo_operand().try_into().unwrap()));
                InstructionResult::Continue
            }
        };

        self.instruction_pointer = match result {
            InstructionResult::Continue => self.instruction_pointer + 2,
            InstructionResult::Jump(target) => target,
            InstructionResult::Halt => self.program.len(),
        };
    }

    fn instruction(&self) -> Instruction {
        num::FromPrimitive::from_usize(self.program[self.instruction_pointer]).unwrap()
    }

    fn combo_operand(&self) -> isize {
        let operand = self.program[self.instruction_pointer + 1];
        match operand {
            0..=3 => operand as isize,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand"),
        }
    }

    fn literal_operand(&self) -> isize {
        self.program[self.instruction_pointer + 1] as isize
    }
}

#[derive(FromPrimitive)]
#[repr(usize)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

enum InstructionResult {
    Continue,
    Jump(usize),
    Halt,
}

pub fn part1(input: &str, _is_sample: bool) -> String {
    let mut output = Vec::new();
    let (registers, program) = parse_vm(input);
    let mut vm = VM::new(registers, &program[..], |num| {
        output.push(num);
        true
    });

    vm.run();

    output.iter().join(",")
}

fn parse_vm(input: &str) -> ([isize; 3], Vec<usize>) {
    let reg_value_regex = Regex::new(r"Register \w: (\d+)").unwrap();
    let (regs, program) = input.split_once("\n\n").unwrap();

    let registers: Vec<isize> = regs
        .lines()
        .map(|line| reg_value_regex.captures(line).unwrap()[1].parse().unwrap())
        .collect();

    let program: Vec<usize> = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    (registers.try_into().unwrap(), program)
}

pub fn part2_brute_force(input: &str, _is_sample: bool) -> isize {
    let (registers, program) = parse_vm(input);

    let chunk_size = 1_000_000_000;
    let mut start = 0;
    loop {
        debug!("{}", start);
        if let Some(result) = (start..(start + chunk_size))
            .into_par_iter()
            .find_first(|i| {
                let mut num_output = 0;
                let mut vm = VM::new([*i, registers[1], registers[2]], &program[..], |num| {
                    if program.get(num_output) == Some(&num.try_into().unwrap()) {
                        num_output += 1;
                        true
                    } else {
                        false
                    }
                });
                vm.run();
                num_output == program.len()
            })
        {
            return result;
        }
        start += chunk_size;
    }
}

fn get_a(index: usize, partial: &HashMap<usize, usize>, program: &[usize]) -> usize {
    if index >= program.len() {
        0
    } else {
        partial
            .get(&index)
            .expect(format!("Can't fetch {}", index).as_str())
            .to_owned()
    }
}

pub fn part2(input: &str, _is_sample: bool) -> String {
    let (registers, program) = parse_vm(input);

    let mut partial = HashMap::new();
    for i in (0..program.len()).rev() {
        let target = program[i];
        for possible in 0..8 {
            let rhs = if possible == 3 {
                possible
            } else {
                get_a(i + (possible ^ 3), &partial, &program[..])
            };
            let output = possible ^ rhs;
            println!("{}: {} ^ {} = {}", i, possible, rhs, output);
            if output == target {
                partial.insert(i, possible);
                break;
            }
        }
        assert!(partial.contains_key(&i), "No solution found for {}", i);
    }

    (0..program.len())
        .map(|i| get_a(i, &partial, &program).to_string())
        .join("")
}
