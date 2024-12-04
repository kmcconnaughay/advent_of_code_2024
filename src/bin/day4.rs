use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../data/day4.txt");
    println!("Day 4 part 1: {}", part1(input));
    println!("Day 4 part 2: {}", part2(input));

    Ok(())
}

struct Offset {
    i: i32,
    j: i32,
}

#[derive(Hash, PartialEq, Eq)]
struct Coordinate {
    i: i32,
    j: i32,
}

impl Coordinate {
    fn plus(&self, offset: &Offset) -> Self {
        Self {
            i: self.i + offset.i,
            j: self.j + offset.j,
        }
    }
}

fn part1(input: &str) -> i32 {
    let word_search: HashMap<Coordinate, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, character)| {
                (
                    Coordinate {
                        i: i as i32,
                        j: j as i32,
                    },
                    character,
                )
            })
        })
        .collect::<HashMap<Coordinate, char>>();

    word_search
        .iter()
        .filter_map(|(coordinate, letter)| {
            if *letter == 'X' {
                Some(coordinate)
            } else {
                None
            }
        })
        .map(|coordinate| {
            XMAS_DIRECTIONS
                .iter()
                .filter(|direction| {
                    word_search
                        .get(&coordinate.plus(&direction[0]))
                        .is_some_and(|letter| *letter == 'M')
                        && word_search
                            .get(&coordinate.plus(&direction[1]))
                            .is_some_and(|letter| *letter == 'A')
                        && word_search
                            .get(&coordinate.plus(&direction[2]))
                            .is_some_and(|letter| *letter == 'S')
                })
                .count() as i32
        })
        .sum()
}

const XMAS_DIRECTIONS: [[Offset; 3]; 8] = [
    [
        Offset { i: 0, j: 1 },
        Offset { i: 0, j: 2 },
        Offset { i: 0, j: 3 },
    ],
    [
        Offset { i: 0, j: -1 },
        Offset { i: 0, j: -2 },
        Offset { i: 0, j: -3 },
    ],
    [
        Offset { i: 1, j: 0 },
        Offset { i: 2, j: 0 },
        Offset { i: 3, j: 0 },
    ],
    [
        Offset { i: -1, j: 0 },
        Offset { i: -2, j: 0 },
        Offset { i: -3, j: 0 },
    ],
    [
        Offset { i: 1, j: 1 },
        Offset { i: 2, j: 2 },
        Offset { i: 3, j: 3 },
    ],
    [
        Offset { i: 1, j: -1 },
        Offset { i: 2, j: -2 },
        Offset { i: 3, j: -3 },
    ],
    [
        Offset { i: -1, j: 1 },
        Offset { i: -2, j: 2 },
        Offset { i: -3, j: 3 },
    ],
    [
        Offset { i: -1, j: -1 },
        Offset { i: -2, j: -2 },
        Offset { i: -3, j: -3 },
    ],
];

fn part2(input: &str) -> i32 {
    let word_search: HashMap<Coordinate, char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, character)| {
                (
                    Coordinate {
                        i: i as i32,
                        j: j as i32,
                    },
                    character,
                )
            })
        })
        .collect::<HashMap<Coordinate, char>>();

    word_search
        .iter()
        .filter_map(|(coordinate, letter)| {
            if *letter == 'A' {
                Some(coordinate)
            } else {
                None
            }
        })
        .filter(|coordinate| {
            let num_matches = X_MAS_DIRECTIONS
                .iter()
                .filter(|offsets| {
                    word_search
                        .get(&coordinate.plus(&offsets[0]))
                        .is_some_and(|letter| *letter == 'M')
                        && word_search
                            .get(&coordinate.plus(&offsets[1]))
                            .is_some_and(|letter| *letter == 'S')
                })
                .count();

            num_matches == 2
        })
        .count() as i32
}

const X_MAS_DIRECTIONS: [[Offset; 2]; 4] = [
    [Offset { i: -1, j: -1 }, Offset { i: 1, j: 1 }],
    [Offset { i: 1, j: 1 }, Offset { i: -1, j: -1 }],
    [Offset { i: 1, j: -1 }, Offset { i: -1, j: 1 }],
    [Offset { i: -1, j: 1 }, Offset { i: 1, j: -1 }],
];

const _X_MAS_CHARS: [char; 2] = ['M', 'S'];

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_counts_xmas() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT), 18);
        Ok(())
    }

    #[test]
    fn part2_counts_x_mas() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT), 9);
        Ok(())
    }
}
