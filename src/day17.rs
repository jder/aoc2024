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

pub fn _part2_brute_force(input: &str, _is_sample: bool) -> isize {
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

fn get_a(bit_index: usize, partial: &usize, program: &[usize]) -> usize {
    if bit_index >= program.len() * 3 {
        0
    } else {
        (*partial >> bit_index) % 8
    }
}

fn set_a(bit_index: usize, partial: &mut usize, value: usize) {
    let mask = 0b111 << bit_index;
    *partial = (*partial & !mask) | (value << bit_index);
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let (registers, program) = parse_vm(input);

    let mut partial = 0usize;
    let solved = solve(program.len() - 1, &mut partial, &program);
    assert!(solved);

    let mut output = Vec::new();
    let mut vm = VM::new(
        [partial as isize, registers[1], registers[2]],
        &program[..],
        |num| {
            output.push(num);
            true
        },
    );

    vm.run();

    debug!(
        "running with {partial} produced {}",
        output.iter().join(",")
    );

    partial
}

fn solve(i: usize, partial: &mut usize, program: &[usize]) -> bool {
    let target = program[i];
    for possible in 0..8 {
        // This was reverse-engineered from my input, yours might be different
        let rhs = if possible == 3 {
            possible
        } else {
            get_a(i * 3 + (possible ^ 3), &partial, &program[..])
        };
        let output = possible ^ rhs;
        debug!("trying {i} {possible} {output} {rhs}");
        if output == target {
            debug!("trying {i} {possible}");
            set_a(i * 3, partial, possible);
            if i > 0 {
                if solve(i - 1, partial, program) {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    return false;
}
