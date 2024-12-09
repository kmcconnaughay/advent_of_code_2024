use std::collections::HashSet;

use anyhow::anyhow;
use ndarray::Array2;
use nom::{
    branch::alt,
    character::{self, complete::line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let map = parse(input)?;
    let mut starting_position = [0, 0];
    map.indexed_iter().position(|((i, j), element)| {
        if *element == Tile::GuardStart {
            starting_position = [i, j];
            true
        } else {
            false
        }
    });
    let simulation_result = simulate_guard(&map, &starting_position, false);

    match simulation_result {
        SimulationResult::LeavesMap {
            num_visited_tiles,
            path: _,
        } => Ok(num_visited_tiles),
        SimulationResult::Loops => Err(anyhow!("Guard unexpectedly loops")),
    }
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let map = parse(input)?;
    let mut starting_position = [0, 0];
    map.indexed_iter().position(|((i, j), element)| {
        if *element == Tile::GuardStart {
            starting_position = [i, j];
            true
        } else {
            false
        }
    });
    let unmodified_simulation = simulate_guard(&map, &starting_position, true);
    let mut proposed_obstacles: HashSet<[usize; 2]> = HashSet::new();

    match unmodified_simulation {
        SimulationResult::LeavesMap {
            num_visited_tiles: _,
            path,
        } => {
            for (position, direction) in &path[..(path.len() - 1)] {
                let obstacle_position = step(position, direction);
                let next_tile = &map[(obstacle_position[0], obstacle_position[1])];
                if *next_tile == Tile::Obstacle || *next_tile == Tile::GuardStart {
                    continue;
                }
                let mut modified_map = map.clone();
                modified_map[(obstacle_position[0], obstacle_position[1])] = Tile::Obstacle;
                if simulate_guard(&modified_map, &starting_position, false)
                    == SimulationResult::Loops
                {
                    proposed_obstacles.insert(obstacle_position);
                }
            }
        }
        SimulationResult::Loops => return Err(anyhow!("Unexpectedly looped")),
    };

    Ok(proposed_obstacles.len())
}

fn simulate_guard(
    map: &Array2<Tile>,
    starting_position: &[usize; 2],
    record_path: bool,
) -> SimulationResult {
    let mut position = *starting_position;
    let mut direction = Direction::Up;
    let mut visited_tiles: Array2<u8> = Array2::zeros(map.dim());
    let mut path = Vec::from([(position, direction)]);

    while !leaving_map(map, &position, &direction) {
        let next_position = step(&position, &direction);
        match map[(next_position[0], next_position[1])] {
            Tile::Empty | Tile::GuardStart => position = next_position,
            Tile::Obstacle => direction = direction.turn_right(),
        }
        if record_path {
            path.push((position, direction));
        }
        if visited_tiles[(position[0], position[1])] & direction.get_bit_mask() != 0 {
            return SimulationResult::Loops;
        } else {
            visited_tiles[(position[0], position[1])] |= direction.get_bit_mask();
        }
    }

    SimulationResult::LeavesMap {
        num_visited_tiles: visited_tiles.iter().filter(|mask| **mask != 0).count(),
        path,
    }
}

#[derive(PartialEq, Eq)]
enum SimulationResult {
    LeavesMap {
        num_visited_tiles: usize,
        path: Vec<([usize; 2], Direction)>,
    },
    Loops,
}

fn leaving_map(map: &Array2<Tile>, position: &[usize; 2], direction: &Direction) -> bool {
    match direction {
        Direction::Up => position[0] == 0,
        Direction::Down => position[0] == map.dim().0 - 1,
        Direction::Left => position[1] == 0,
        Direction::Right => position[1] == map.dim().1 - 1,
    }
}

fn parse(input: &str) -> anyhow::Result<Array2<Tile>> {
    let (_, map) = separated_list1(line_ending, many1(alt((empty, obstacle, guard_start))))(input)
        .map_err(|e| anyhow!("{}", e))?;
    let num_rows = map.len();
    let num_cols = map[0].len();
    let flattened = map.into_iter().flatten().collect::<Vec<Tile>>();
    Array2::from_shape_vec((num_rows, num_cols), flattened).map_err(|e| anyhow!("{}", e))
}

fn empty(input: &str) -> IResult<&str, Tile> {
    value(Tile::Empty, character::complete::char('.'))(input)
}

fn obstacle(input: &str) -> IResult<&str, Tile> {
    value(Tile::Obstacle, character::complete::char('#'))(input)
}

fn guard_start(input: &str) -> IResult<&str, Tile> {
    value(Tile::GuardStart, character::complete::char('^'))(input)
}

fn step(position: &[usize; 2], direction: &Direction) -> [usize; 2] {
    match direction {
        Direction::Up => [position[0] - 1, position[1]],
        Direction::Down => [position[0] + 1, position[1]],
        Direction::Left => [position[0], position[1] - 1],
        Direction::Right => [position[0], position[1] + 1],
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    GuardStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn get_bit_mask(&self) -> u8 {
        match self {
            Direction::Up => 0b00000001,
            Direction::Down => 0b00000010,
            Direction::Left => 0b00000100,
            Direction::Right => 0b00001000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_counts_distinct_guard_positions() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 41);
        Ok(())
    }

    #[test]
    fn part2_counts_proposed_obstacles() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 6);
        Ok(())
    }
}
