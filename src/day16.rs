use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::anyhow;
use ndarray::Array2;
use nom::{
    branch::alt,
    character::{self, complete::line_ending},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let (_, map) = parse(input).map_err(|e| anyhow!("Failed to parse input: {e}"))?;
    match (find(&map, &Tile::Start), find(&map, &Tile::End)) {
        (Some(start), Some(end)) => dijkstra(&map, &start, &end)
            .map(|(minimum_cost, _covered_tiles)| minimum_cost)
            .ok_or(anyhow!("No valid path from start to end")),
        _ => Err(anyhow!("Unable to locate start and end")),
    }
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, map) = parse(input).map_err(|e| anyhow!("Failed to parse input: {e}"))?;

    match (find(&map, &Tile::Start), find(&map, &Tile::End)) {
        (Some(start), Some(end)) => dijkstra(&map, &start, &end)
            .map(|(_minimum_cost, covered_tiles)| covered_tiles)
            .ok_or(anyhow!("No valid path from start to end")),
        _ => Err(anyhow!("Unable to locate start and end")),
    }
}

fn find(map: &Map, tile: &Tile) -> Option<Position> {
    map.indexed_iter().find_map(|(position, map_tile)| {
        if *tile == *map_tile {
            Some(position)
        } else {
            None
        }
    })
}

fn dijkstra(map: &Map, start: &Position, end: &Position) -> Option<(u32, usize)> {
    let initial_posture = Posture {
        position: *start,
        facing: Facing::East,
    };
    let mut costs = HashMap::<Posture, u32>::with_capacity(map.len() * 2);
    costs.insert(initial_posture, 0);

    let mut priority_queue = BinaryHeap::with_capacity(map.len() * 2);
    priority_queue.push(SearchState {
        posture: initial_posture,
        cost: 0,
    });

    let mut parents = HashMap::<Posture, HashSet<Posture>>::new();

    while let Some(SearchState { posture, cost }) = priority_queue.pop() {
        if posture.position == *end {
            continue;
        }

        if cost > *costs.get(&posture).unwrap_or(&u32::MAX) {
            continue;
        }

        let mut adjacencies = vec![
            SearchState {
                posture: Posture {
                    position: posture.position,
                    facing: posture.facing.rotate_right(),
                },
                cost: cost + 1000,
            },
            SearchState {
                posture: Posture {
                    position: posture.position,
                    facing: posture.facing.rotate_left(),
                },
                cost: cost + 1000,
            },
        ];

        if let Some(stepped_forward) = posture.step_forward(map) {
            adjacencies.push(SearchState {
                posture: stepped_forward,
                cost: cost + 1,
            });
        }

        for adjaceny in adjacencies {
            let existing_cost = costs.get(&adjaceny.posture).unwrap_or(&u32::MAX);
            match adjaceny.cost.cmp(existing_cost) {
                Ordering::Less => {
                    let adjacency_parents = parents.entry(adjaceny.posture).or_default();
                    adjacency_parents.clear();
                    adjacency_parents.insert(posture);
                    costs.insert(adjaceny.posture, adjaceny.cost);
                    priority_queue.push(adjaceny);
                }
                Ordering::Equal => {
                    let adjacency_parents = parents.entry(adjaceny.posture).or_default();
                    adjacency_parents.insert(posture);
                }
                Ordering::Greater => (),
            };
        }
    }

    let minimum_cost = costs
        .iter()
        .filter_map(|(posture, cost)| {
            if posture.position == *end {
                Some(cost)
            } else {
                None
            }
        })
        .min()
        .copied();

    match minimum_cost {
        Some(min_c) => {
            let mut parent_stack = costs
                .iter()
                .filter_map(|(posture, cost)| {
                    if posture.position == *end && min_c == *cost {
                        Some(posture)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect::<Vec<Posture>>();
            let mut covered_tiles = HashSet::<Position>::new();

            while let Some(parent) = parent_stack.pop() {
                covered_tiles.insert(parent.position);
                if let Some(grandparents) = parents.get(&parent) {
                    parent_stack.extend(grandparents);
                }
            }

            Some((min_c, covered_tiles.len()))
        }
        None => None,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    posture: Posture,
    cost: u32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.posture.cmp(&other.posture))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Posture {
    position: Position,
    facing: Facing,
}

impl Posture {
    fn step_forward(&self, map: &Map) -> Option<Posture> {
        let next_position = match self.facing {
            Facing::North => (self.position.0 - 1, self.position.1),
            Facing::South => (self.position.0 + 1, self.position.1),
            Facing::East => (self.position.0, self.position.1 + 1),
            Facing::West => (self.position.0, self.position.1 - 1),
        };

        match map[next_position] {
            Tile::Start | Tile::End | Tile::Empty => Some(Posture {
                position: next_position,
                facing: self.facing,
            }),
            Tile::Wall => None,
        }
    }
}

type Map = Array2<Tile>;

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn rotate_right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
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

fn parse(input: &str) -> IResult<&str, Map> {
    all_consuming(map(separated_list1(line_ending, map_row), |map| {
        let num_rows = map.len();
        let num_cols = map.first().map_or(0, |line| line.len());
        Array2::from_shape_vec(
            (num_rows, num_cols),
            map.iter().flatten().cloned().collect(),
        )
        .expect("Failed to convert input to Array2")
    }))(input)
}

fn map_row(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(alt((start, end, empty, wall)))(input)
}

fn start(input: &str) -> IResult<&str, Tile> {
    value(Tile::Start, character::complete::char('S'))(input)
}

fn end(input: &str) -> IResult<&str, Tile> {
    value(Tile::End, character::complete::char('E'))(input)
}

fn empty(input: &str) -> IResult<&str, Tile> {
    value(Tile::Empty, character::complete::char('.'))(input)
}

fn wall(input: &str) -> IResult<&str, Tile> {
    value(Tile::Wall, character::complete::char('#'))(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_returns_cost_of_cheapest_maze_solve() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 11048);
        Ok(())
    }

    #[test]
    fn part2_returns_number_of_suitable_seats() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 64);
        Ok(())
    }
}
