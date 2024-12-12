use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    character::{self, complete::multispace1},
    combinator::map,
    multi::separated_list1,
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let (_, mut stones) = parse(input).map_err(|e| anyhow!("Failed to parse input: {e}"))?;
    expand(&mut stones, 25);
    Ok(stones.values().sum())
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let (_, mut stones) = parse(input).map_err(|e| anyhow!("Failed to parse input: {e}"))?;
    expand(&mut stones, 75);
    Ok(stones.values().sum())
}

fn expand(stones: &mut HashMap<u64, u64>, iterations: u32) {
    for _ in 0..iterations {
        let mut next_expansion = HashMap::default();
        stones.iter().for_each(|(stone, count)| {
            rule1(stone)
                .or_else(|| rule2(stone))
                .unwrap_or_else(|| rule3(stone))
                .iter()
                .for_each(|new_stone| *next_expansion.entry(*new_stone).or_default() += count)
        });

        *stones = next_expansion;
    }
}

fn rule1(stone: &u64) -> Option<Vec<u64>> {
    if *stone == 0 {
        Some(vec![1])
    } else {
        None
    }
}

fn rule2(stone: &u64) -> Option<Vec<u64>> {
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
        let scaling_factor = 10u64.pow(num_digits / 2);
        let left = stone / scaling_factor;
        let right = stone - (left * scaling_factor);
        Some(vec![left, right])
    } else {
        None
    }
}

fn rule3(stone: &u64) -> Vec<u64> {
    vec![stone * 2024]
}

fn parse(input: &str) -> IResult<&str, HashMap<u64, u64>> {
    map(
        separated_list1(multispace1, character::complete::u64),
        |stones| {
            stones.iter().fold(HashMap::default(), |mut acc, stone| {
                *acc.entry(*stone).or_default() += 1;
                acc
            })
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "125 17";

    #[test]
    fn part1_counts_blinking_stones() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 55312);
        Ok(())
    }
}
