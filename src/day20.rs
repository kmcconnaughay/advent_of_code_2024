use std::cmp::{max, min};

use anyhow::anyhow;
use itertools::Itertools;
use ndarray::Array2;
use nom::{
    branch::alt,
    character::{self, complete::line_ending},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};
use rayon::prelude::*;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse(input)?;
    num_cheats(
        &grid, /* min_saving= */ 100, /* max_cheat_cost= */ 2,
    )
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse(input)?;
    num_cheats(
        &grid, /* min_saving= */ 100, /* max_cheat_cost= */ 20,
    )
}

fn num_cheats(
    grid: &Array2<Tile>,
    min_time_saved: usize,
    max_cheat_cost: usize,
) -> anyhow::Result<usize> {
    let original_path = get_path(
        &grid,
        &find_tile(&grid, &Tile::Start)?,
        &find_tile(&grid, &Tile::End)?,
    )?;
    Ok(original_path[..(original_path.len() - 1)]
        .par_iter()
        .enumerate()
        .map(|(start_index, cheat_start)| {
            original_path[(start_index + 1)..]
                .iter()
                .enumerate()
                .filter(|(end_index, cheat_end)| {
                    let base_cost = end_index + 1;
                    let cheat_cost = manhattan_distance(cheat_start, cheat_end);
                    cheat_cost <= max_cheat_cost && base_cost - cheat_cost >= min_time_saved
                })
                .count()
        })
        .sum())
}

fn get_path(
    grid: &Array2<Tile>,
    start: &Position,
    end: &Position,
) -> anyhow::Result<Vec<Position>> {
    let mut path = vec![*start];
    let mut previous_position = None;
    let mut current_position = *start;

    while current_position != *end {
        let next_position = find_next_position(grid, &previous_position, &current_position)?;

        path.push(next_position);
        previous_position = Some(current_position);
        current_position = next_position;
    }

    Ok(path)
}

fn find_next_position(
    grid: &Array2<Tile>,
    previous_position: &Option<Position>,
    current_position: &Position,
) -> anyhow::Result<Position> {
    let up = (current_position.0 - 1, current_position.1);
    let down = (current_position.0 + 1, current_position.1);
    let left = (current_position.0, current_position.1 - 1);
    let right = (current_position.0, current_position.1 + 1);

    [up, down, left, right]
        .into_iter()
        .filter(|neighbor| {
            grid[*neighbor] != Tile::Wall
                && previous_position.map_or(true, |previous| *neighbor != previous)
        })
        .exactly_one()
        .map_err(|e| {
            anyhow!(
                "Expected exactly one neighbor of position ({}, {}): {}",
                current_position.0,
                current_position.1,
                e
            )
        })
}

fn manhattan_distance(a: &Position, b: &Position) -> usize {
    max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
}

fn find_tile(grid: &Array2<Tile>, target: &Tile) -> anyhow::Result<Position> {
    grid.indexed_iter()
        .filter_map(|(position, tile)| {
            if *tile == *target {
                Some(position)
            } else {
                None
            }
        })
        .exactly_one()
        .map_err(|e| anyhow!("Expected exactly one {target} to exist in grid: {e}"))
}

fn parse(input: &str) -> anyhow::Result<Array2<Tile>> {
    let (_, grid) = map(
        all_consuming(separated_list1(line_ending, many1(Tile::parse))),
        |raw| {
            let nrows = raw.len();
            let ncols = raw[0].len();
            Array2::from_shape_vec((nrows, ncols), raw.into_iter().flatten().collect())
                .expect("Failed to convert input to Array2")
        },
    )(input)
    .map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok(grid)
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Tile> {
        alt((
            value(Tile::Start, character::complete::char('S')),
            value(Tile::End, character::complete::char('E')),
            value(Tile::Empty, character::complete::char('.')),
            value(Tile::Wall, character::complete::char('#')),
        ))(input)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => 'S',
                Tile::End => 'E',
                Tile::Empty => '.',
                Tile::Wall => '#',
            }
        )
    }
}

type Position = (usize, usize);

#[cfg(test)]
mod tests {
    use super::{num_cheats, parse};

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn num_cheats_returns_number_of_valid_shortcuts() -> anyhow::Result<()> {
        let grid = parse(INPUT)?;
        assert_eq!(
            num_cheats(&grid, /* min_saving= */ 2, /* max_cheat_cost= */ 2)?,
            44
        );
        assert_eq!(
            num_cheats(&grid, /* min_saving= */ 20, /* max_cheat_cost= */ 2)?,
            5
        );
        assert_eq!(
            num_cheats(&grid, /* min_saving= */ 76, /* max_cheat_cost= */ 20)?,
            3
        );
        assert_eq!(
            num_cheats(&grid, /* min_saving= */ 50, /* max_cheat_cost= */ 20)?,
            285
        );
        Ok(())
    }
}
