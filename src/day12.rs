use std::collections::HashSet;

use ndarray::Array2;

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let regions = segment(&map);
    regions
        .iter()
        .map(|region| area(region) * perimeter(&map, region))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let regions = segment(&map);
    regions
        .iter()
        .map(|region| area(region) * edges(&map, region))
        .sum()
}

type Coordinate = (usize, usize);
type Region = HashSet<Coordinate>;

fn area(region: &Region) -> usize {
    region.len()
}

fn perimeter(map: &Array2<char>, region: &Region) -> usize {
    region
        .iter()
        .map(|coordinate| {
            [
                top(map, coordinate),
                right(map, coordinate),
                bottom(map, coordinate),
                left(map, coordinate),
            ]
            .iter()
            .filter(|neighbor| {
                neighbor
                    .map(|neighbor| map[neighbor] != map[*coordinate])
                    .unwrap_or(true)
            })
            .count()
        })
        .sum()
}

fn edges(map: &Array2<char>, region: &Region) -> usize {
    let mut num_edges = 0;

    for coordinate in region {
        let tl = in_region(region, top_left(map, coordinate));
        let t = in_region(region, top(map, coordinate));
        let tr = in_region(region, top_right(map, coordinate));
        let l = in_region(region, left(map, coordinate));
        let r = in_region(region, right(map, coordinate));
        let bl = in_region(region, bottom_left(map, coordinate));
        let b = in_region(region, bottom(map, coordinate));
        let br = in_region(region, bottom_right(map, coordinate));

        if (!t && !r) || (t && r && !tr) {
            num_edges += 1;
        }

        if (!r && !b) || (r && b && !br) {
            num_edges += 1
        }

        if (!b && !l) || (b && l && !bl) {
            num_edges += 1
        }

        if (!l && !t) || (l && t && !tl) {
            num_edges += 1
        }
    }

    num_edges
}

fn in_region(region: &HashSet<(usize, usize)>, coordinate: Option<Coordinate>) -> bool {
    coordinate
        .map(|neighbor| region.contains(&neighbor))
        .unwrap_or(false)
}

fn segment(map: &Array2<char>) -> Vec<Region> {
    let mut visited = HashSet::<Coordinate>::new();
    let mut regions = Vec::<Region>::new();

    for (coordinate, plant_type) in map.indexed_iter() {
        if !visited.insert(coordinate) {
            continue;
        }

        let region = fill(map, coordinate, plant_type);
        visited.extend(region.clone());
        regions.push(region);
    }

    regions
}

fn fill(map: &Array2<char>, coordinate: Coordinate, plant_type: &char) -> Region {
    let mut region = Region::default();
    let mut stack = vec![coordinate];

    while let Some(new_coordinate) = stack.pop() {
        if map[new_coordinate] != *plant_type {
            continue;
        }

        if !region.insert(new_coordinate) {
            continue;
        }

        stack.extend(orthogonal_neighbors(map, &new_coordinate));
    }

    region
}

fn orthogonal_neighbors(map: &Array2<char>, coordinate: &Coordinate) -> Vec<Coordinate> {
    top(map, coordinate)
        .into_iter()
        .chain(right(map, coordinate))
        .chain(bottom(map, coordinate))
        .chain(left(map, coordinate))
        .collect()
}

fn top_left(_map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row > 0 && *col > 0 {
        Some((row - 1, col - 1))
    } else {
        None
    }
}

fn top(_map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row > 0 {
        Some((row - 1, *col))
    } else {
        None
    }
}

fn top_right(map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row > 0 && *col < map.dim().1 - 1 {
        Some((row - 1, col + 1))
    } else {
        None
    }
}

fn left(_map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *col > 0 {
        Some((*row, col - 1))
    } else {
        None
    }
}

fn right(map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *col < map.dim().1 - 1 {
        Some((*row, col + 1))
    } else {
        None
    }
}

fn bottom_left(map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row < map.dim().0 - 1 && *col > 0 {
        Some((row + 1, col - 1))
    } else {
        None
    }
}

fn bottom(map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row < map.dim().0 - 1 {
        Some((row + 1, *col))
    } else {
        None
    }
}

fn bottom_right(map: &Array2<char>, (row, col): &Coordinate) -> Option<Coordinate> {
    if *row < map.dim().0 - 1 && *col < map.dim().1 - 1 {
        Some((row + 1, col + 1))
    } else {
        None
    }
}

fn parse(input: &str) -> Array2<char> {
    let raw: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = raw.len();
    let num_cols = raw.first().map_or(0, |line| line.len());
    Array2::from_shape_vec(
        (num_rows, num_cols),
        raw.iter().flatten().cloned().collect(),
    )
    .expect("Failed to transmute input to Array2")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_returns_total_price_of_fencing() {
        assert_eq!(part1(INPUT), 1930);
    }

    #[test]
    fn part2_returns_discount_price_of_fencing() {
        assert_eq!(part2(INPUT), 1206);
    }
}
