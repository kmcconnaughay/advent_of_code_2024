use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut computer = parse(input)?;
    computer.execute()?;
    Ok(computer.emit_output())
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let mut computer = parse(input)?;
    computer.find_quine()
}

#[derive(Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn find_quine(&mut self) -> anyhow::Result<u64> {
        let mut search_space = vec![0];

        for target_value in self.program.clone().iter().rev() {
            let mut next_search_space = vec![];
            for needle in &search_space {
                for remainder in 0..8 {
                    let candidate = (needle * 8) + remainder;
                    self.reset();
                    self.register_a = candidate;
                    self.execute()?;
                    if self.output[0] == *target_value {
                        next_search_space.push(candidate);
                    }
                }
            }
            search_space = next_search_space;
        }

        search_space
            .into_iter()
            .min()
            .ok_or(anyhow!("Unable to find suitable value for register a"))
    }

    fn reset(&mut self) {
        self.register_a = 0;
        self.register_b = 0;
        self.register_c = 0;
        self.instruction_pointer = 0;
        self.output.clear();
    }

    fn emit_output(&self) -> String {
        self.output.iter().map(|out| out.to_string()).join(",")
    }

    fn execute(&mut self) -> anyhow::Result<()> {
        while self.step()? {}
        Ok(())
    }

    fn step(&mut self) -> anyhow::Result<bool> {
        if self.instruction_pointer + 1 >= self.program.len() {
            return Ok(false);
        }

        let opcode = Opcode::parse(self.program[self.instruction_pointer]);
        let operand = self.program[self.instruction_pointer + 1];

        let operand_value = match opcode {
            Opcode::Adv | Opcode::Bst | Opcode::Out | Opcode::Bdv | Opcode::Cdv => {
                self.combo_operand(operand)
            }
            Opcode::Bxl | Opcode::Jnz | Opcode::Bxc => operand as u64,
        };

        match opcode {
            Opcode::Adv => {
                self.register_a /= 2_u64.pow(operand_value.try_into()?);
                self.instruction_pointer += 2;
            }
            Opcode::Bxl => {
                self.register_b ^= operand_value;
                self.instruction_pointer += 2;
            }
            Opcode::Bst => {
                self.register_b = operand_value % 8;
                self.instruction_pointer += 2;
            }
            Opcode::Jnz => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand_value as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            Opcode::Bxc => {
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            }
            Opcode::Out => {
                self.output.push((operand_value % 8) as u8);
                self.instruction_pointer += 2;
            }
            Opcode::Bdv => {
                self.register_b = self.register_a / (2_u64.pow(operand_value.try_into()?));
                self.instruction_pointer += 2;
            }
            Opcode::Cdv => {
                self.register_c = self.register_a / (2_u64.pow(operand_value.try_into()?));
                self.instruction_pointer += 2;
            }
        };

        Ok(true)
    }

    fn combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Operand 7 is reserved and will not appear in valid programs"),
            _ => panic!("Unrecognized operand: {operand}"),
        }
    }
}

enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn parse(opcode: u8) -> Opcode {
        match opcode {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Unrecognized opcode: {opcode}"),
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Computer> {
    let (_, computer) = all_consuming(map(
        separated_pair(registers, line_ending, program),
        |((register_a, register_b, register_c), program)| Computer {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
            output: vec![],
        },
    ))(input)
    .map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok(computer)
}

fn registers(input: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
        terminated(register, line_ending),
        terminated(register, line_ending),
        terminated(register, line_ending),
    ))(input)
}

fn register(input: &str) -> IResult<&str, u64> {
    preceded(
        alt((
            tag("Register A: "),
            tag("Register B: "),
            tag("Register C: "),
        )),
        character::complete::u64,
    )(input)
}

fn program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tag("Program: "),
        separated_list1(character::complete::char(','), character::complete::u8),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{parse, part1};

    #[test]
    pub fn find_quine_returns_suitable_register_values() -> anyhow::Result<()> {
        let input = include_str!("../inputs/day17.txt");

        let mut computer = parse(input)?;
        let candidate = computer.find_quine()?;
        computer.reset();
        computer.register_a = candidate;
        computer.execute()?;

        assert_eq!(computer.output, computer.program);
        Ok(())
    }

    #[test]
    pub fn part1_returns_program_output() -> anyhow::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        assert_eq!(part1(input)?, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    pub fn bst_writes_to_register_b() -> anyhow::Result<()> {
        let input = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";

        let mut computer = parse(input)?;
        computer.execute()?;

        assert_eq!(computer.register_b, 1);
        Ok(())
    }

    #[test]
    pub fn out_writes_operands_to_output() -> anyhow::Result<()> {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

        let mut computer = parse(input)?;
        computer.execute()?;

        assert_eq!(computer.emit_output(), "0,1,2");
        Ok(())
    }

    #[test]
    pub fn jnz_jumps() -> anyhow::Result<()> {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let mut computer = parse(input)?;
        computer.execute()?;

        assert_eq!(computer.register_a, 0);
        assert_eq!(computer.emit_output(), "4,2,5,6,7,7,7,7,3,1,0");
        Ok(())
    }

    #[test]
    pub fn bxl_stores_bitwise_or_in_register_b() -> anyhow::Result<()> {
        let input = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";

        let mut computer = parse(input)?;
        computer.execute()?;

        assert_eq!(computer.register_b, 26);
        Ok(())
    }

    #[test]
    pub fn bxc_stores_bitwise_or_in_register_b() -> anyhow::Result<()> {
        let input = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";

        let mut computer = parse(input)?;
        computer.execute()?;

        assert_eq!(computer.register_b, 44354);
        Ok(())
    }
}
