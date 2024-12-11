use std::iter::zip;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{line_ending, space1},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let (_, calibration_equations) =
        parse(input).map_err(|e| anyhow!("Unable to parse input: {}", e))?;

    let operators = [Operator::Add, Operator::Multiply];

    let total_calibration_result = calibration_equations
        .par_iter()
        .filter_map(|(solution, operands)| {
            let num_operators = operands.len() - 1;
            (0..num_operators)
                .map(|_| operators)
                .multi_cartesian_product()
                .any(|operator_sequence| {
                    let value = zip(operands[1..].iter(), operator_sequence.iter()).fold(
                        operands[0],
                        |acc, (operand, operator)| match operator {
                            Operator::Add => acc + operand,
                            Operator::Multiply => acc * operand,
                            Operator::Concatenate => panic!("Unexpected concatenate"),
                        },
                    );

                    value == *solution
                })
                .then_some(solution)
        })
        .sum();

    Ok(total_calibration_result)
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let (_, calibration_equations) =
        parse(input).map_err(|e| anyhow!("Unable to parse input: {}", e))?;
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];

    let total_calibration_result = calibration_equations
        .par_iter()
        .filter_map(|(solution, operands)| {
            let num_operators = operands.len() - 1;
            (0..num_operators)
                .map(|_| operators)
                .multi_cartesian_product()
                .any(|operator_sequence| {
                    let value = zip(operands[1..].iter(), operator_sequence.iter()).fold(
                        operands[0],
                        |acc, (operand, operator)| match operator {
                            Operator::Add => acc + operand,
                            Operator::Multiply => acc * operand,
                            Operator::Concatenate => concatenate(&acc, operand),
                        },
                    );

                    value == *solution
                })
                .then_some(solution)
        })
        .sum();

    Ok(total_calibration_result)
}

fn concatenate(a: &u64, b: &u64) -> u64 {
    a * (10_u64.pow(b.ilog10() + 1)) + b
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            character::complete::u64,
            tag(": "),
            separated_list1(space1, character::complete::u64),
        ),
    )(input)
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_returns_total_calibration_result() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 3749);
        Ok(())
    }

    #[test]
    fn part2_returns_total_calibration_result() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 11387);
        Ok(())
    }
}
