use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character;
use nom::character::complete::anychar;
use nom::combinator::{map, value};
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../data/day3.txt");
    println!("Day 3 part 1: {}", part1(input)?);
    println!("Day 3 part 2: {}", part2(input)?);

    Ok(())
}

#[derive(Clone)]

enum Instruction {
    Mul { multiplicand: i64, multiplier: i64 },
    Do,
    Dont,
}

fn mul_parser(input: &str) -> IResult<&str, Instruction> {
    map(
        delimited(
            tag("mul("),
            separated_pair(
                character::complete::i64,
                character::complete::char(','),
                character::complete::i64,
            ),
            character::complete::char(')'),
        ),
        |(multiplicand, multiplier)| Instruction::Mul {
            multiplicand,
            multiplier,
        },
    )(input)
}

fn do_parser(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Do, tag("do()"))(input)
}

fn dont_parser(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Dont, tag("don't()"))(input)
}

fn instruction_parser(input: &str) -> IResult<&str, Instruction> {
    alt((mul_parser, do_parser, dont_parser))(input)
}

fn memory_parser(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(
        many_till(anychar, instruction_parser),
        |(_, instructions)| instructions,
    ))(input)
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, instructions) =
        memory_parser(input).map_err(|e| anyhow!("Failed to parse memory: {}", e))?;
    let solution = instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul {
                multiplicand,
                multiplier,
            } => multiplicand * multiplier,
            _ => 0,
        })
        .sum();
    Ok(solution)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, instructions) =
        memory_parser(input).map_err(|e| anyhow!("Failed to parse memory: {}", e))?;

    let mut solution = 0;
    let mut muls_enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul {
                multiplicand,
                multiplier,
            } => {
                if muls_enabled {
                    solution += multiplicand * multiplier
                }
            }
            Instruction::Do => muls_enabled = true,
            Instruction::Dont => muls_enabled = false,
        }
    }

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_identifies_and_adds_valid_muls() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input)?, 161);
        Ok(())
    }

    #[test]
    fn part2_toggles_muls() -> anyhow::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input)?, 48);
        Ok(())
    }
}
