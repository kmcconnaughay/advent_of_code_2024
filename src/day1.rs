use std::collections::HashMap;
use std::iter::zip;

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let mut group1: Vec<i64> = Vec::new();
    let mut group2: Vec<i64> = Vec::new();

    for line in input.lines() {
        if let Some((location_one, location_two)) = line.split_once("   ") {
            group1.push(location_one.parse::<i64>()?);
            group2.push(location_two.parse::<i64>()?);
        }
    }

    group1.sort();
    group2.sort();

    Ok(zip(group1, group2)
        .map(|(location_one, location_two)| {
            return (location_one - location_two).abs();
        })
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let mut group1: Vec<i64> = Vec::new();
    let mut group2: Vec<i64> = Vec::new();

    for line in input.lines() {
        if let Some((location_one, location_two)) = line.split_once("   ") {
            group1.push(location_one.parse::<i64>()?);
            group2.push(location_two.parse::<i64>()?);
        }
    }

    let mut counter: HashMap<i64, i64> = HashMap::new();
    for location in group2 {
        let count = counter.get(&location).unwrap_or(&0);
        counter.insert(location, count + 1);
    }

    let mut similarity = 0;

    for location in group1 {
        let count = counter.get(&location).unwrap_or(&0);
        similarity += location * count;
    }

    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_computes_total_distance() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 11);
        Ok(())
    }

    #[test]
    fn part2_computes_similarity_score() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 31);
        Ok(())
    }
}
