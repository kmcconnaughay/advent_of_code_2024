use std::collections::{HashMap, HashSet};
use std::iter::zip;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let secrets = parse(input)?;
    Ok(secrets
        .into_par_iter()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = evolve(secret);
            }
            secret
        })
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<i32> {
    let secrets = parse(input)?;
    let mut sequence_prices = HashMap::<(i32, i32, i32, i32), i32>::with_capacity(10_usize.pow(4));

    for mut secret in secrets {
        let mut evolutions = Vec::with_capacity(2001);
        evolutions.push(secret);
        for _ in 0..2000 {
            secret = evolve(secret);
            evolutions.push(secret);
        }

        let prices = evolutions
            .iter()
            .map(|secret| price(*secret))
            .collect::<Vec<i32>>();

        let changes = prices
            .iter()
            .tuple_windows()
            .map(|(price_a, price_b)| price_b - price_a)
            .collect::<Vec<i32>>();

        let sequences: Vec<(i32, i32, i32, i32)> = changes.into_iter().tuple_windows().collect();
        let mut seen = HashSet::<(i32, i32, i32, i32)>::with_capacity(2000);
        zip(sequences.into_iter(), prices[4..].iter()).for_each(|(sequence, price)| {
            if seen.insert(sequence) {
                sequence_prices
                    .entry(sequence)
                    .and_modify(|existing| *existing += price)
                    .or_insert(*price);
            }
        });
    }

    sequence_prices
        .values()
        .max()
        .copied()
        .ok_or(anyhow!("No max value found"))
}

const PRUNE_MASK: u64 = 2_u64.pow(24) - 1;

fn evolve(mut secret: u64) -> u64 {
    secret ^= secret << 6;
    secret &= PRUNE_MASK;
    secret ^= secret >> 5;
    secret &= PRUNE_MASK;
    secret ^= secret << 11;
    secret &= PRUNE_MASK;

    secret
}

fn price(secret: u64) -> i32 {
    (secret % 10) as i32
}

fn parse(input: &str) -> anyhow::Result<Vec<u64>> {
    let parse_result: IResult<&str, Vec<u64>> =
        all_consuming(separated_list1(line_ending, complete::u64))(input);
    let (_, secrets) = parse_result.map_err(|e| anyhow!("Failed to parse input: {}", e))?;
    Ok(secrets)
}

#[cfg(test)]
mod tests {
    use super::{evolve, part1, part2};
    use rstest::*;

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(16495136, 527345)]
    #[case(527345, 704524)]
    #[case(704524, 1553684)]
    #[case(1553684, 12683156)]
    #[case(12683156, 11100544)]
    #[case(11100544, 12249484)]
    #[case(12249484, 7753432)]
    #[case(7753432, 5908254)]
    fn evolve_returns_new_secret(#[case] secret: u64, #[case] evolved: u64) {
        assert_eq!(evolve(secret), evolved);
    }

    #[test]
    fn part1_returns_sum_of_evolved_secrets() -> anyhow::Result<()> {
        let input = "1
10
100
2024";
        assert_eq!(part1(input)?, 37327623);
        Ok(())
    }

    #[test]
    fn part2_returns_maximum_number_of_bananas() -> anyhow::Result<()> {
        let input = "1
2
3
2024";
        assert_eq!(part2(input)?, 23);
        Ok(())
    }
}
