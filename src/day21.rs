use itertools::Itertools;
use memoize::memoize;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let codes = parse(input)?;
    Ok(codes.into_iter().map(|code| code.complexity(2)).sum())
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let codes = parse(input)?;
    Ok(codes.into_iter().map(|code| code.complexity(25)).sum())
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
#[derive(Clone, Copy, PartialEq, Eq)]
enum Num {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Num {
    fn x_position(&self) -> isize {
        match self {
            Num::One | Num::Four | Num::Seven => 0,
            Num::Zero | Num::Two | Num::Five | Num::Eight => 1,
            Num::A | Num::Three | Num::Six | Num::Nine => 2,
        }
    }

    fn y_position(&self) -> isize {
        match self {
            Num::Seven | Num::Eight | Num::Nine => 0,
            Num::Four | Num::Five | Num::Six => 1,
            Num::One | Num::Two | Num::Three => 2,
            Num::Zero | Num::A => 3,
        }
    }

    fn dirs_to(&self, next: &Num) -> Vec<Dir> {
        if self == next {
            return vec![Dir::A];
        }

        let mut order = [Dir::Left, Dir::Down, Dir::Up, Dir::Right];
        if (matches!(self, Num::Zero | Num::A) && matches!(next, Num::One | Num::Four | Num::Seven))
            || (matches!(self, Num::One | Num::Four | Num::Seven)
                && matches!(next, Num::Zero | Num::A))
        {
            order = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
        }

        let delta_x = next.x_position() - self.x_position();
        let delta_y = next.y_position() - self.y_position();
        let mut result = Vec::with_capacity(delta_x.unsigned_abs() + delta_y.unsigned_abs() + 1);

        for dir in order.iter() {
            match dir {
                Dir::Up if delta_y < 0 => {
                    result.extend(std::iter::repeat(Dir::Up).take(delta_y.unsigned_abs()))
                }
                Dir::Down if delta_y > 0 => {
                    result.extend(std::iter::repeat(Dir::Down).take(delta_y as usize))
                }
                Dir::Left if delta_x < 0 => {
                    result.extend(std::iter::repeat(Dir::Left).take(delta_x.unsigned_abs()))
                }
                Dir::Right if delta_x > 0 => {
                    result.extend(std::iter::repeat(Dir::Right).take(delta_x as usize))
                }
                _ => {}
            }
        }

        result.push(Dir::A);
        result
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Dir {
    fn dirs_to(&self, next: &Dir) -> Vec<Dir> {
        match (self, next) {
            (Dir::Up, Dir::Up) => vec![Dir::A],
            (Dir::Up, Dir::Down) => vec![Dir::Down, Dir::A],
            (Dir::Up, Dir::Left) => vec![Dir::Down, Dir::Left, Dir::A],
            (Dir::Up, Dir::Right) => vec![Dir::Down, Dir::Right, Dir::A],
            (Dir::Up, Dir::A) => vec![Dir::Right, Dir::A],

            (Dir::Down, Dir::Up) => vec![Dir::Up, Dir::A],
            (Dir::Down, Dir::Down) => vec![Dir::A],
            (Dir::Down, Dir::Left) => vec![Dir::Left, Dir::A],
            (Dir::Down, Dir::Right) => vec![Dir::Right, Dir::A],
            (Dir::Down, Dir::A) => vec![Dir::Up, Dir::Right, Dir::A],

            (Dir::Left, Dir::Up) => vec![Dir::Right, Dir::Up, Dir::A],
            (Dir::Left, Dir::Down) => vec![Dir::Right, Dir::A],
            (Dir::Left, Dir::Left) => vec![Dir::A],
            (Dir::Left, Dir::Right) => vec![Dir::Right, Dir::Right, Dir::A],
            (Dir::Left, Dir::A) => vec![Dir::Right, Dir::Right, Dir::Up, Dir::A],

            (Dir::Right, Dir::Up) => vec![Dir::Left, Dir::Up, Dir::A],
            (Dir::Right, Dir::Down) => vec![Dir::Left, Dir::A],
            (Dir::Right, Dir::Left) => vec![Dir::Left, Dir::Left, Dir::A],
            (Dir::Right, Dir::Right) => vec![Dir::A],
            (Dir::Right, Dir::A) => vec![Dir::Up, Dir::A],

            (Dir::A, Dir::Up) => vec![Dir::Left, Dir::A],
            (Dir::A, Dir::Down) => vec![Dir::Left, Dir::Down, Dir::A],
            (Dir::A, Dir::Left) => vec![Dir::Down, Dir::Left, Dir::Left, Dir::A],
            (Dir::A, Dir::Right) => vec![Dir::Down, Dir::A],
            (Dir::A, Dir::A) => vec![Dir::A],
        }
    }
}

// A sequence of base 10 digits followed by the letter 'A'.
struct Code {
    nums: Vec<Num>,
}

impl Code {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            many1(alt((
                value(Num::Zero, complete::char('0')),
                value(Num::One, complete::char('1')),
                value(Num::Two, complete::char('2')),
                value(Num::Three, complete::char('3')),
                value(Num::Four, complete::char('4')),
                value(Num::Five, complete::char('5')),
                value(Num::Six, complete::char('6')),
                value(Num::Seven, complete::char('7')),
                value(Num::Eight, complete::char('8')),
                value(Num::Nine, complete::char('9')),
                value(Num::A, complete::char('A')),
            ))),
            |nums| Code { nums },
        )(input)
    }

    fn numeric_value(self) -> u64 {
        u64::from(self)
    }

    fn complexity(self, num_robots: u32) -> u64 {
        let initial_dirs = prepend(Num::A, self.nums.as_slice())
            .tuple_windows()
            .flat_map(|(from, to)| from.dirs_to(&to))
            .collect::<Vec<Dir>>();

        sequence_length(initial_dirs, num_robots) * self.numeric_value()
    }
}

impl From<Code> for u64 {
    fn from(code: Code) -> Self {
        let mut result = 0;

        for num in code.nums[..code.nums.len().saturating_sub(1)].iter() {
            result *= 10;
            match num {
                Num::One => result += 1,
                Num::Two => result += 2,
                Num::Three => result += 3,
                Num::Four => result += 4,
                Num::Five => result += 5,
                Num::Six => result += 6,
                Num::Seven => result += 7,
                Num::Eight => result += 8,
                Num::Nine => result += 9,
                _ => (),
            }
        }

        result
    }
}

#[memoize]
fn sequence_length(directions: Vec<Dir>, num_robots: u32) -> u64 {
    if num_robots == 0 {
        return directions.len() as u64;
    }

    prepend(Dir::A, directions.as_slice())
        .tuple_windows()
        .map(|(from, to)| sequence_length(from.dirs_to(&to), num_robots - 1))
        .sum()
}

fn prepend<T: Copy>(prefix: T, s: &[T]) -> impl Iterator<Item = T> + '_ {
    std::iter::once(prefix).chain(s.iter().copied())
}

fn parse(input: &str) -> anyhow::Result<Vec<Code>> {
    let parse_result = all_consuming(separated_list1(line_ending, Code::parse))(input);
    let (_, codes) = parse_result.map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(codes)
}

#[cfg(test)]
mod tests {
    use super::{part1, Code};
    use anyhow::anyhow;
    use rstest::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_returns_code_complexity() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 126384);
        Ok(())
    }

    #[rstest]
    #[case("029A", 1972)]
    #[case("980A", 58800)]
    #[case("179A", 12172)]
    #[case("456A", 29184)]
    #[case("379A", 24256)]
    fn complexity(#[case] input: &str, #[case] expected: u64) -> anyhow::Result<()> {
        let (_, code) = Code::parse(input).map_err(|e| anyhow!("{}", e))?;
        assert_eq!(code.complexity(2), expected);
        Ok(())
    }
}
