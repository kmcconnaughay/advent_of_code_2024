use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::anyhow;
use ndarray::Array2;
use nom::{
    character::{self, complete::line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub const MAP_DIMS: (usize, usize) = (71, 71);

pub fn part1(input: &str, map_dims: (usize, usize), num_fallen: usize) -> anyhow::Result<u32> {
    let corrupted_positions = parse(input)?;
    let memory_space = make_memory_space(map_dims, &corrupted_positions[0..num_fallen]);
    let start: Position = (0, 0);
    let end: Position = (map_dims.0 - 1, map_dims.1 - 1);

    dijkstra(&memory_space, &start, &end).ok_or(anyhow!(
        "Unable to find path from {:?} to {:?}",
        start,
        end
    ))
}

pub fn part2(input: &str, map_dims: (usize, usize)) -> anyhow::Result<Position> {
    let corrupted_positions = parse(input)?;
    let mut memory_space = make_memory_space(map_dims, &[]);
    let start: Position = (0, 0);
    let end: Position = (map_dims.0 - 1, map_dims.1 - 1);

    corrupted_positions
        .into_iter()
        .find(|fallen| {
            memory_space[*fallen] = 1;
            !connected(&memory_space, &start, &end)
        })
        .ok_or(anyhow!("No fallen byte made memory space impassable"))
}

fn dijkstra(memory_space: &MemorySpace, start: &Position, end: &Position) -> Option<u32> {
    let mut costs = HashMap::<Position, u32>::new();
    costs.insert(*start, 0);

    let mut priority_queue = BinaryHeap::<SearchState>::new();
    priority_queue.push(SearchState {
        position: *start,
        cost: 0,
    });

    while let Some(search_state) = priority_queue.pop() {
        if search_state.position == *end {
            continue;
        }

        if memory_space[search_state.position] == 1 {
            continue;
        }

        if search_state.cost > *costs.get(&search_state.position).unwrap_or(&u32::MAX) {
            continue;
        }

        let adjacent_cost = search_state.cost + 1;
        for adjacent in adjacent_positions(memory_space, &search_state.position) {
            if adjacent_cost < *costs.get(&adjacent).unwrap_or(&u32::MAX) {
                costs.insert(adjacent, adjacent_cost);
                priority_queue.push(SearchState {
                    position: adjacent,
                    cost: adjacent_cost,
                });
            }
        }
    }

    costs.get(end).copied()
}

fn connected(memory_space: &MemorySpace, start: &Position, end: &Position) -> bool {
    let mut visited = Array2::<bool>::from_elem(memory_space.dim(), false);
    let mut stack = vec![*start];

    while let Some(position) = stack.pop() {
        if position == *end {
            return true;
        }

        if visited[position] {
            continue;
        }

        visited[position] = true;

        for adjacent in adjacent_positions(memory_space, &position) {
            stack.push(adjacent);
        }
    }

    false
}

fn adjacent_positions(memory_space: &MemorySpace, position: &Position) -> Vec<Position> {
    let mut adjacent = vec![];
    let (row, col) = position;
    if *row > 0 {
        adjacent.push((row - 1, *col));
    }

    if *row < memory_space.nrows() - 1 {
        adjacent.push((row + 1, *col));
    }

    if *col > 0 {
        adjacent.push((*row, col - 1));
    }

    if *col < memory_space.ncols() - 1 {
        adjacent.push((*row, col + 1));
    }

    adjacent
        .into_iter()
        .filter(|a| memory_space[*a] == 0)
        .collect()
}

fn make_memory_space(dims: (usize, usize), corrupted_positions: &[Position]) -> MemorySpace {
    let mut memory_space = MemorySpace::zeros(dims);
    for position in corrupted_positions {
        memory_space[*position] = 1;
    }
    memory_space
}

#[derive(PartialEq, Eq)]
struct SearchState {
    position: Position,
    cost: u32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Position = (usize, usize);

type MemorySpace = Array2<u8>;

fn parse(input: &str) -> anyhow::Result<Vec<Position>> {
    let (_, positions) = separated_list1(line_ending, position)(input)
        .map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok(positions)
}

fn position(input: &str) -> IResult<&str, Position> {
    map(
        separated_pair(
            character::complete::u8,
            character::complete::char(','),
            character::complete::u8,
        ),
        |(x, y)| (x as usize, y as usize),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_returns_length_of_shortest_path() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT, (7, 7), 12)?, 22);
        Ok(())
    }

    #[test]
    fn part2_returns_first_byte_that_makes_traversal_impossible() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT, (7, 7))?, (6, 1));
        Ok(())
    }
}
