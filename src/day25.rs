use anyhow::anyhow;
use itertools::{Either, Itertools};
use nom::{
    branch::alt,
    character::complete::{self, line_ending, one_of},
    combinator::{all_consuming, map},
    multi::{count, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let (locks, keys) = parse(input)?;

    Ok(locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.can_open_with(key))
        .count())
}

fn parse(input: &str) -> anyhow::Result<(Vec<Lock>, Vec<Key>)> {
    let parse_result = all_consuming(separated_list1(line_ending, LockOrKey::parse))(input);
    let (_, locks_or_keys) = parse_result.map_err(|e| anyhow!("{}", e))?;

    Ok(locks_or_keys
        .into_iter()
        .partition_map(|lock_or_key| match lock_or_key {
            LockOrKey::Lock(lock) => Either::Left(lock),
            LockOrKey::Key(key) => Either::Right(key),
        }))
}

const LOCK_WIDTH: usize = 5;
const LOCK_HEIGHT: usize = 5;

enum LockOrKey {
    Lock(Lock),
    Key(Key),
}

impl LockOrKey {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Lock::parse, LockOrKey::Lock),
            map(Key::parse, LockOrKey::Key),
        ))(input)
    }
}

struct Lock {
    pins: [u8; LOCK_WIDTH],
}

impl Lock {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(delimited(filled_row, heights, empty_row), |pins| Lock {
            pins,
        })(input)
    }

    fn can_open_with(&self, key: &Key) -> bool {
        let overflows = (0..LOCK_WIDTH).any(|i| self.pins[i] + key.bitings[i] > LOCK_HEIGHT as u8);
        !overflows
    }
}

struct Key {
    bitings: [u8; LOCK_WIDTH],
}

impl Key {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(delimited(empty_row, heights, filled_row), |bitings| Key {
            bitings,
        })(input)
    }
}

fn filled_row(input: &str) -> IResult<&str, ()> {
    map(
        terminated(count(complete::char('#'), LOCK_WIDTH), line_ending),
        |_| (),
    )(input)
}

fn empty_row(input: &str) -> IResult<&str, ()> {
    map(
        terminated(count(complete::char('.'), LOCK_WIDTH), line_ending),
        |_| (),
    )(input)
}

fn heights(input: &str) -> IResult<&str, [u8; LOCK_WIDTH]> {
    let row = terminated(count(one_of(".#"), LOCK_WIDTH), line_ending);
    let rows = count(row, LOCK_HEIGHT);

    map(rows, |rs| {
        let mut heights = [0; LOCK_WIDTH];
        for row in rs {
            for (i, &c) in row.iter().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }
        heights
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn part1_returns_number_of_keys_that_fit() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 3);
        Ok(())
    }
}
