#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
use std::str::FromStr;

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ComboOperand(isize);

impl ComboOperand {
    fn value(self, registers: [isize; 3]) -> isize {
        match self.0 {
            0..=3 => self.0,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Adv(ComboOperand),
    Bxl(isize),
    Bst(ComboOperand),
    Jnz(isize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Operation {
    fn new(instruction: isize, operand: isize) -> Self {
        match instruction {
            0 => Operation::Adv(ComboOperand(operand)),
            1 => Operation::Bxl(operand),
            2 => Operation::Bst(ComboOperand(operand)),
            3 => Operation::Jnz(operand),
            4 => Operation::Bxc,
            5 => Operation::Out(ComboOperand(operand)),
            6 => Operation::Bdv(ComboOperand(operand)),
            7 => Operation::Cdv(ComboOperand(operand)),
            _ => unreachable!(),
        }
    }

    fn execute(self, pc: usize, registers: [isize; 3]) -> (usize, [isize; 3], Option<isize>) {
        match self {
            Self::Adv(operand) => (
                pc + 1,
                [
                    registers[0] / 2isize.pow(operand.value(registers) as u32),
                    registers[1],
                    registers[2],
                ],
                None,
            ),
            Self::Bxl(operand) => (
                pc + 1,
                [registers[0], registers[1] ^ operand, registers[2]],
                None,
            ),
            Self::Bst(operand) => (
                pc + 1,
                [registers[0], operand.value(registers) & 0b111, registers[2]],
                None,
            ),
            Self::Jnz(operand) => {
                if registers[0] != 0 {
                    (operand as usize, registers, None)
                } else {
                    (pc + 1, registers, None)
                }
            }
            Self::Bxc => (
                pc + 1,
                [registers[0], registers[1] ^ registers[2], registers[2]],
                None,
            ),
            Self::Out(operand) => (pc + 1, registers, Some(operand.value(registers) & 0b111)),
            Self::Bdv(operand) => (
                pc + 1,
                [
                    registers[0],
                    registers[0] / 2isize.pow(operand.value(registers) as u32),
                    registers[2],
                ],
                None,
            ),
            Self::Cdv(operand) => (
                pc + 1,
                [
                    registers[0],
                    registers[1],
                    registers[0] / 2isize.pow(operand.value(registers) as u32),
                ],
                None,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct System {
    registers: [isize; 3],
    program: Vec<Operation>,
    output: Vec<isize>,
    program_string: String,
}

impl System {
    fn execute_program(&mut self) {
        let mut pc = 0;
        while pc < self.program.len() {
            let operation = self.program[pc];
            let (new_pc, new_registers, output) = operation.execute(pc, self.registers);
            pc = new_pc;
            self.registers = new_registers;
            if let Some(output) = output {
                self.output.push(output);
            }
        }
    }

    fn get_output_string(&self) -> String {
        self.output
            .iter()
            .map(isize::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (register_part, program_part) = s.split_once("\n\n").unwrap();

        let registers = register_part
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect::<Vec<isize>>();

        let program_string = program_part.split_once(": ").unwrap().1;
        let numbers: Vec<isize> = program_string
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let program = numbers
            .chunks(2)
            .map(|chunk| Operation::new(chunk[0], chunk[1]))
            .collect();

        Ok(System {
            registers: [registers[0], registers[1], registers[2]],
            program,
            program_string: program_string.to_string(),
            output: Vec::new(),
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let mut system: System = input.parse().unwrap();
        system.execute_program();
        Some(system.get_output_string().into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let system: System = input.parse().unwrap();

        let mut min = 0;
        let mut max = 0;
        let mut curr = 1;

        while min == 0 {
            curr *= 2;
            let mut system_clone = system.clone();
            system_clone.registers[0] = curr;
            system_clone.execute_program();
            if system_clone.get_output_string().len() == system.program_string.len() {
                min = curr;
            }
        }
        while max == 0 {
            curr *= 2;
            let mut system_clone = system.clone();
            system_clone.registers[0] = curr;
            system_clone.execute_program();
            if system_clone.get_output_string().len() != system.program_string.len() {
                max = curr;
            }
        }

        let mut cur = min;
        let mut offset_size = 1;
        let mut max_similar = 0;
        let mut output = String::new();
        while cur <= max && max_similar < 31 {
            cur += offset_size;

            let mut system_clone = system.clone();
            system_clone.registers[0] = cur;
            system_clone.execute_program();
            output = system_clone.get_output_string();

            let mut similar_length = 0;
            for (a, b) in output.chars().zip(system.program_string.chars()) {
                if a == b {
                    similar_length += 1;
                } else {
                    break;
                }
            }

            if similar_length > max_similar {
                if max_similar > 1 {
                    // This is probably specific to my input
                    offset_size <<= 3;
                }
                max_similar = similar_length;
            }
        }
        if output == system.program_string {
            Some((cur as usize).into())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 17;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some("4,6,3,5,6,3,5,2,1,0".into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some("1,5,3,0,2,5,2,5,3".into()));
    }

    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(108_107_566_389_757.into()));
    }
}
