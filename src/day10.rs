use std::collections::{HashSet, VecDeque};

use anyhow::anyhow;
use itertools::Itertools;
use ndarray::Array2;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let topographical_map = parse(input)?;

    let total_trail_score = topographical_map
        .indexed_iter()
        .filter_map(|(position, height)| {
            if *height != 0 {
                return None;
            }

            Some(score_trailhead(&topographical_map, position))
        })
        .sum();

    Ok(total_trail_score)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let topographical_map = parse(input)?;

    let total_trail_score = topographical_map
        .indexed_iter()
        .filter_map(|(position, height)| {
            if *height != 0 {
                return None;
            }

            Some(rate_trailhead(&topographical_map, position))
        })
        .sum();

    Ok(total_trail_score)
}

#[derive(Debug)]
struct SearchState {
    position: (usize, usize),
    height: u32,
}

fn score_trailhead(topographical_map: &ndarray::Array2<u32>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::default();
    queue.push_back(SearchState {
        position: start,
        height: topographical_map[start],
    });
    let mut trail_ends = HashSet::<(usize, usize)>::default();

    while let Some(state) = queue.pop_front() {
        if state.height == 9 {
            trail_ends.insert(state.position);
        }
        let (row, col) = state.position;
        let next_height = state.height + 1;

        if row > 0 && topographical_map[(row - 1, col)] == next_height {
            queue.push_back(SearchState {
                position: (row - 1, col),
                height: next_height,
            });
        }

        if row < topographical_map.dim().0 - 1 && topographical_map[(row + 1, col)] == next_height {
            queue.push_back(SearchState {
                position: (row + 1, col),
                height: next_height,
            });
        }

        if col > 0 && topographical_map[(row, col - 1)] == next_height {
            queue.push_back(SearchState {
                position: (row, col - 1),
                height: next_height,
            });
        }

        if col < topographical_map.dim().1 - 1 && topographical_map[(row, col + 1)] == next_height {
            queue.push_back(SearchState {
                position: (row, col + 1),
                height: next_height,
            });
        }
    }

    trail_ends.len()
}

fn rate_trailhead(topographical_map: &ndarray::Array2<u32>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::default();
    queue.push_back(SearchState {
        position: start,
        height: topographical_map[start],
    });
    let mut rating = 0;

    while let Some(state) = queue.pop_front() {
        if state.height == 9 {
            rating += 1;
        }
        let (row, col) = state.position;
        let next_height = state.height + 1;

        if row > 0 && topographical_map[(row - 1, col)] == next_height {
            queue.push_back(SearchState {
                position: (row - 1, col),
                height: next_height,
            });
        }

        if row < topographical_map.dim().0 - 1 && topographical_map[(row + 1, col)] == next_height {
            queue.push_back(SearchState {
                position: (row + 1, col),
                height: next_height,
            });
        }

        if col > 0 && topographical_map[(row, col - 1)] == next_height {
            queue.push_back(SearchState {
                position: (row, col - 1),
                height: next_height,
            });
        }

        if col < topographical_map.dim().1 - 1 && topographical_map[(row, col + 1)] == next_height {
            queue.push_back(SearchState {
                position: (row, col + 1),
                height: next_height,
            });
        }
    }

    rating
}

fn parse(input: &str) -> anyhow::Result<Array2<u32>> {
    let raw: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| {
                    height.to_digit(10).ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        anyhow!("Unable to parse character into base 10 digit: {height}"),
                    ))
                })
                .try_collect()
        })
        .try_collect()?;

    let num_rows = raw.len();
    let num_cols = raw[0].len();
    Array2::from_shape_vec(
        (num_rows, num_cols),
        raw.iter().flatten().cloned().collect(),
    )
    .map_err(|e| anyhow!("{}", e))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    pub fn part1_scores_trailheads() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 36);
        Ok(())
    }

    #[test]
    pub fn part2_rates_trailheads() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 81);
        Ok(())
    }
}
