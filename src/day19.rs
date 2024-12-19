use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let (towels, designs) = parse(input)?;
    let mut cache = HashMap::new();

    Ok(designs
        .iter()
        .map(|design| num_possibilities(design, &towels, &mut cache))
        .filter(|num| *num > 0)
        .count())
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let (towels, designs) = parse(input)?;
    let mut cache = HashMap::new();

    Ok(designs
        .iter()
        .map(|design| num_possibilities(design, &towels, &mut cache))
        .sum())
}

fn num_possibilities<'d>(
    design: &'d str,
    towels: &[&str],
    cache: &mut HashMap<&'d str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(num) = cache.get(design) {
        return *num;
    }

    let num = towels
        .iter()
        .filter(|towel| design.starts_with(*towel))
        .map(|towel| num_possibilities(&design[towel.len()..], towels, cache))
        .sum();

    cache.insert(design, num);
    num
}

fn parse(input: &str) -> anyhow::Result<(Vec<&str>, Vec<&str>)> {
    let (_, (towels, designs)) =
        all_consuming(separated_pair(towels, many1(line_ending), designs))(input)
            .map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok((towels, designs))
}

fn towels(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn designs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_returns_number_of_possible_designs() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 6);
        Ok(())
    }

    #[test]
    fn part2_returns_count_of_all_possible_designs() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 16);
        Ok(())
    }
}
