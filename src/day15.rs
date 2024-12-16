use std::collections::{HashMap, VecDeque};

use anyhow::anyhow;
use ndarray::Array2;
use nom::{
    branch::alt,
    character::{self, complete::line_ending},
    combinator::{map, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, (mut warehouse, directions)) =
        parse(input).map_err(|e| anyhow!("Unable to parse input: {e}"))?;

    let mut robot = robot_position(&warehouse);
    for direction in directions {
        robot = move_robot(&mut warehouse, &robot, &direction);
    }

    Ok(warehouse
        .indexed_iter()
        .filter_map(|(position, tile)| {
            if *tile == Tile::Kasten {
                Some(gps_coordinate(&position))
            } else {
                None
            }
        })
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, (mut warehouse, directions)) =
        parse(&widen(input)).map_err(|e| anyhow!("Unable to parse input: {e}"))?;

    let mut robot = robot_position(&warehouse);
    for direction in directions {
        robot = move_robot(&mut warehouse, &robot, &direction);
    }

    Ok(warehouse
        .indexed_iter()
        .filter_map(|(position, tile)| {
            if *tile == Tile::KastenLeft {
                Some(gps_coordinate(&position))
            } else {
                None
            }
        })
        .sum())
}

fn robot_position(warehouse: &Array2<Tile>) -> Position {
    warehouse
        .indexed_iter()
        .find_map(|(position, tile)| {
            if *tile == Tile::Robot {
                Some(position)
            } else {
                None
            }
        })
        .expect("The warehouse has no robot")
}

fn move_robot(warehouse: &mut Array2<Tile>, robot: &Position, direction: &Direction) -> Position {
    let mut moving_tiles = HashMap::<Position, Tile>::new();
    let mut queue = VecDeque::<Position>::new();
    let mut move_allowed = true;

    moving_tiles.insert(*robot, warehouse[*robot]);
    queue.push_back(*robot);

    while let Some(current_position) = queue.pop_front() {
        let current_tile = warehouse[current_position];
        match current_tile {
            Tile::Empty => (),
            Tile::Wall => {
                move_allowed = false;
                break;
            }
            Tile::Robot | Tile::Kasten => {
                moving_tiles.insert(current_position, current_tile);
                queue.push_back(get_next_position(&current_position, direction));
            }
            Tile::KastenLeft => {
                let right_position = (current_position.0, current_position.1 + 1);
                let right_tile = warehouse[right_position];
                moving_tiles.insert(current_position, current_tile);
                moving_tiles.insert(right_position, right_tile);

                let next_left_position = get_next_position(&current_position, direction);
                let next_right_position = get_next_position(&right_position, direction);

                match direction {
                    Direction::Left => queue.push_back(next_left_position),
                    Direction::Right => queue.push_back(next_right_position),
                    Direction::Up | Direction::Down => {
                        queue.push_back(next_left_position);
                        queue.push_back(next_right_position);
                    }
                }
            }
            Tile::KastenRight => {
                let left_position = (current_position.0, current_position.1 - 1);
                let left_tile = warehouse[left_position];
                moving_tiles.insert(left_position, left_tile);
                moving_tiles.insert(current_position, current_tile);

                let next_left_position = get_next_position(&left_position, direction);
                let next_right_position = get_next_position(&current_position, direction);

                match direction {
                    Direction::Left => queue.push_back(next_left_position),
                    Direction::Right => queue.push_back(next_right_position),
                    Direction::Up | Direction::Down => {
                        queue.push_back(next_left_position);
                        queue.push_back(next_right_position);
                    }
                }
            }
        }
    }

    if move_allowed {
        for position in moving_tiles.keys() {
            warehouse[*position] = Tile::Empty;
        }

        for (position, tile) in moving_tiles.iter() {
            warehouse[get_next_position(position, direction)] = *tile;
        }

        get_next_position(robot, direction)
    } else {
        *robot
    }
}

fn get_next_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::Up => (position.0 - 1, position.1),
        Direction::Right => (position.0, position.1 + 1),
        Direction::Down => (position.0 + 1, position.1),
        Direction::Left => (position.0, position.1 - 1),
    }
}

fn gps_coordinate((row, col): &Position) -> usize {
    100 * row + col
}

type Position = (usize, usize);

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Robot,
    Empty,
    Wall,
    Kasten,
    KastenLeft,
    KastenRight,
}

fn widen(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '#' => "##".to_owned(),
            'O' => "[]".to_owned(),
            '@' => "@.".to_owned(),
            '.' => "..".to_owned(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn parse(input: &str) -> IResult<&str, (Array2<Tile>, Vec<Direction>)> {
    separated_pair(warehouse, line_ending, directions)(input)
}

fn warehouse(input: &str) -> IResult<&str, Array2<Tile>> {
    map(many1(terminated(tiles, line_ending)), |warehouse| {
        let num_rows = warehouse.len();
        let num_cols = warehouse.first().map_or(0, |line| line.len());
        Array2::from_shape_vec(
            (num_rows, num_cols),
            warehouse.iter().flatten().cloned().collect(),
        )
        .expect("Failed to convert input to Array2")
    })(input)
}

fn tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(alt((wall, kasten, kasten_left, kasten_right, empty, robot)))(input)
}

fn wall(input: &str) -> IResult<&str, Tile> {
    value(Tile::Wall, character::complete::char('#'))(input)
}

fn kasten(input: &str) -> IResult<&str, Tile> {
    value(Tile::Kasten, character::complete::char('O'))(input)
}

fn kasten_left(input: &str) -> IResult<&str, Tile> {
    value(Tile::KastenLeft, character::complete::char('['))(input)
}

fn kasten_right(input: &str) -> IResult<&str, Tile> {
    value(Tile::KastenRight, character::complete::char(']'))(input)
}

fn empty(input: &str) -> IResult<&str, Tile> {
    value(Tile::Empty, character::complete::char('.'))(input)
}

fn robot(input: &str) -> IResult<&str, Tile> {
    value(Tile::Robot, character::complete::char('@'))(input)
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(
        many1(terminated(many1(direction), line_ending)),
        |directions| directions.iter().flatten().cloned().collect(),
    )(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((up, right, down, left))(input)
}

fn up(input: &str) -> IResult<&str, Direction> {
    value(Direction::Up, character::complete::char('^'))(input)
}

fn right(input: &str) -> IResult<&str, Direction> {
    value(Direction::Right, character::complete::char('>'))(input)
}

fn down(input: &str) -> IResult<&str, Direction> {
    value(Direction::Down, character::complete::char('v'))(input)
}

fn left(input: &str) -> IResult<&str, Direction> {
    value(Direction::Left, character::complete::char('<'))(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn part1_sums_gps_coordinates() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 10092);
        Ok(())
    }

    #[test]
    fn part2_sums_gps_coordinates() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 9021);
        Ok(())
    }
}
