use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;
use std::error::Error;
use std::iter::zip;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../data/day1.txt");
    let mut group1: Vec<Location> = Vec::new();
    let mut group2: Vec<Location> = Vec::new();

    for line in input.lines() {
        let (_, (location_one, location_two)) = locations_parser(line)?;
        group1.push(location_one);
        group2.push(location_two);
    }

    group1.sort();
    group2.sort();

    println!("Day 1 part 1: {}", part1(&group1, &group2));
    println!("Day 1 part 2: {}", part2(&group1, &group2));

    Ok(())
}

fn locations_parser(input: &str) -> IResult<&str, (Location, Location)> {
    let (remaining, (id_one, id_two)) = separated_pair(i64, tag("   "), i64)(input)?;
    Ok((
        remaining,
        (Location { id: id_one }, Location { id: id_two }),
    ))
}

fn part1(group1: &[Location], group2: &[Location]) -> i64 {
    zip(group1, group2)
        .map(|(location_one, location_two)| location_one.distance(&location_two))
        .sum()
}

fn part2(group1: &[Location], group2: &[Location]) -> i64 {
    let mut counter: HashMap<&Location, i64> = HashMap::new();
    for location in group2 {
        let count = counter.get(location).unwrap_or(&0);
        counter.insert(location, count + 1);
    }

    let mut similarity = 0;

    for location in group1 {
        let count = counter.get(location).unwrap_or(&0);
        similarity += location.id * count;
    }

    similarity
}

trait Metric<T> {
    fn distance(&self, other: &T) -> i64;
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Location {
    id: i64,
}

impl Metric<Location> for Location {
    fn distance(&self, other: &Location) -> i64 {
        return (self.id - other.id).abs();
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, part1, part2};

    const GROUP1: [Location;6] = [
        Location { id: 1 },
        Location { id: 2 },
        Location { id: 3 },
        Location { id: 3 },
        Location { id: 3 },
        Location { id: 4 },
    ];

    const GROUP2: [Location;6] = [
        Location { id: 3 },
        Location { id: 3 },
        Location { id: 3 },
        Location { id: 4 },
        Location { id: 5 },
        Location { id: 9 },
    ];

    #[test]
    fn part1_computes_total_distance() {
        let total_distance: i64 = part1(&GROUP1, &GROUP2);

        assert_eq!(total_distance, 11);
    }

    #[test]
    fn part2_computes_similarity_score() {
        let similarity: i64 = part2(&GROUP1, &GROUP2);

        assert_eq!(similarity, 31);
    }
}
