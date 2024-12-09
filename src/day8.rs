use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let (map_dims, mut map) = parse(input);
    map.sort_by(|a, b| a.frequency.cmp(&b.frequency));

    let unique_antinode_locations = map
        .into_iter()
        .chunk_by(|antenna| antenna.frequency)
        .into_iter()
        .flat_map(|(_frequency, group)| {
            let antennas = group.collect::<Vec<Antenna>>();
            antennas
                .iter()
                .cartesian_product(&antennas)
                .filter(|(a, b)| a != b)
                .flat_map(|(a, b)| a.antinodes(b))
                .collect::<Vec<Coordinate>>()
        })
        .filter(|antinode| in_bounds(antinode, &map_dims))
        .collect::<HashSet<Coordinate>>();

    unique_antinode_locations.len()
}

pub fn part2(input: &str) -> usize {
    let (map_dims, mut map) = parse(input);
    map.sort_by(|a, b| a.frequency.cmp(&b.frequency));

    let unique_antinode_locations = map
        .into_iter()
        .chunk_by(|antenna| antenna.frequency)
        .into_iter()
        .flat_map(|(_frequency, group)| {
            let antennas = group.collect::<Vec<Antenna>>();
            antennas
                .iter()
                .cartesian_product(&antennas)
                .filter(|(a, b)| a != b)
                .flat_map(|(a, b)| a.resonant_antinodes(b, &map_dims))
                .collect::<Vec<Coordinate>>()
        })
        .collect::<HashSet<Coordinate>>();

    unique_antinode_locations.len()
}

type Coordinate = [isize; 2];

#[derive(Clone, Debug, PartialEq)]
struct Antenna {
    frequency: char,
    coordinate: Coordinate,
}

impl Antenna {
    fn delta(&self, other: &Self) -> Coordinate {
        [
            self.coordinate[0] - other.coordinate[0],
            self.coordinate[1] - other.coordinate[1],
        ]
    }

    fn antinodes(&self, other: &Self) -> [Coordinate; 2] {
        let delta = self.delta(other);
        let positions = [
            [self.coordinate[0] + delta[0], self.coordinate[1] + delta[1]],
            [
                other.coordinate[0] - delta[0],
                other.coordinate[1] - delta[1],
            ],
        ];

        positions
    }

    fn resonant_antinodes(&self, other: &Self, map_dims: &[isize; 2]) -> Vec<Coordinate> {
        let delta = self.delta(other);
        let mut coordinates = Vec::new();

        let mut current = self.coordinate;
        loop {
            coordinates.push(current);
            current = [current[0] + delta[0], current[1] + delta[1]];

            if !in_bounds(&current, map_dims) {
                break;
            }
        }

        current = other.coordinate;
        loop {
            coordinates.push(current);
            current = [current[0] - delta[0], current[1] - delta[1]];

            if !in_bounds(&current, map_dims) {
                break;
            }
        }

        coordinates
    }
}

fn in_bounds(coordinate: &Coordinate, map_dims: &[isize; 2]) -> bool {
    coordinate[0] >= 0
        && coordinate[0] < map_dims[0]
        && coordinate[1] >= 0
        && coordinate[1] < map_dims[1]
}

fn parse(input: &str) -> (Coordinate, Vec<Antenna>) {
    let num_rows = input.lines().count() as isize;
    let num_cols = input.lines().next().map_or(0, |line| line.len()) as isize;
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().filter_map(move |(col, frequency)| {
                if frequency == '.' {
                    None
                } else {
                    Some(Antenna {
                        frequency: frequency,
                        coordinate: [row as isize, col as isize],
                    })
                }
            })
        })
        .collect::<Vec<Antenna>>();

    ([num_rows, num_cols], antennas)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_returns_number_of_unique_antinode_positions() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn part2_returns_number_of_unique_antinode_positions_with_resonance() {
        assert_eq!(part2(INPUT), 34);
    }
}
