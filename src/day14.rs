use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub const MAP_DIMS: (usize, usize) = (101, 103);

pub fn part1(input: &str, dims: (usize, usize)) -> anyhow::Result<i32> {
    let (_, robots) = parse(input).map_err(|e| anyhow!("Unable to parse input: {e}"))?;

    let quadrant_counts = robots
        .iter()
        .filter_map(|robot| identify_quadrant(&simulate(robot, 100, dims), &dims))
        .fold(HashMap::<Quadrant, i32>::default(), |mut acc, quadrant| {
            *acc.entry(quadrant).or_insert(0) += 1;
            acc
        });

    Ok(quadrant_counts.values().product())
}

pub fn part2(input: &str, dims: (usize, usize)) -> anyhow::Result<i32> {
    let (_, robots) = parse(input).map_err(|e| anyhow!("Unable to parse input: {e}"))?;

    let mut seconds = 0;
    loop {
        seconds += 1;
        let positions = robots
            .iter()
            .map(|robot| simulate(robot, seconds, dims))
            .collect::<Vec<Position>>();
        let unique_positions = positions.iter().cloned().collect::<HashSet<Position>>();

        if positions.len() == unique_positions.len() {
            return Ok(seconds);
        }
    }
}

fn simulate(robot: &Robot, seconds: i32, dims: (usize, usize)) -> Position {
    justify(robot.position + robot.velocity.displacement(seconds), dims)
}

fn justify(new_position: Position, dims: (usize, usize)) -> Position {
    Position {
        x: new_position.x.rem_euclid(dims.0 as i32),
        y: new_position.y.rem_euclid(dims.1 as i32),
    }
}

fn identify_quadrant(position: &Position, dims: &(usize, usize)) -> Option<Quadrant> {
    let width = dims.0 as i32;
    let height = dims.1 as i32;
    let left = position.x < width / 2;
    let right = position.x > (width / 2);
    let top = position.y < height / 2;
    let bottom = position.y > (height / 2);
    if left && top {
        Some(Quadrant::Northwest)
    } else if right && top {
        Some(Quadrant::Northeast)
    } else if left && bottom {
        Some(Quadrant::Southwest)
    } else if right && bottom {
        Some(Quadrant::Southeast)
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Quadrant {
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

struct Robot {
    position: Position,
    velocity: Velocity,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add<Displacement> for Position {
    type Output = Position;

    fn add(self, rhs: Displacement) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn displacement(&self, seconds: i32) -> Displacement {
        Displacement {
            x: self.x * seconds,
            y: self.y * seconds,
        }
    }
}

struct Displacement {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, robot)(input)
}

fn robot(input: &str) -> IResult<&str, Robot> {
    map(
        separated_pair(position, character::complete::char(' '), velocity),
        |(position, velocity)| Robot { position, velocity },
    )(input)
}

fn position(input: &str) -> IResult<&str, Position> {
    map(
        preceded(
            tag("p="),
            separated_pair(
                character::complete::i32,
                character::complete::char(','),
                character::complete::i32,
            ),
        ),
        |(x, y)| Position { x, y },
    )(input)
}

fn velocity(input: &str) -> IResult<&str, Velocity> {
    map(
        preceded(
            tag("v="),
            separated_pair(
                character::complete::i32,
                character::complete::char(','),
                character::complete::i32,
            ),
        ),
        |(x, y)| Velocity { x, y },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    pub fn part1_returns_safety_factor() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT, (11, 7))?, 12);
        Ok(())
    }
}
