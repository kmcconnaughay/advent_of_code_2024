use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::anyhow;
use nom::character;
use nom::character::complete::line_ending;
use nom::multi::fold_many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

type Multimap<K, V> = HashMap<K, HashSet<V>>;

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let (_, (page_ordering_rules, page_updates)) =
        parse(input).map_err(|e| anyhow!("Unable to parse input: {}", e))?;

    Ok(page_updates
        .iter()
        .filter(|update| {
            update.is_sorted_by(|a, b| {
                page_ordering_rules
                    .get(a)
                    .map_or(false, |followers| followers.contains(b))
            })
        })
        .map(|update| update[update.len() / 2])
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    let (_, (page_ordering_rules, mut page_updates)) =
        parse(input).map_err(|e| anyhow!("Unable to parse input: {}", e))?;

    Ok(page_updates
        .iter_mut()
        .filter(|update| {
            !update.is_sorted_by(|a, b| {
                page_ordering_rules
                    .get(a)
                    .map_or(false, |followers| followers.contains(b))
            })
        })
        .map(|update| {
            let pivot_position = update.len() / 2;
            let (_lesser, pivot, _greater) =
                update.select_nth_unstable_by(pivot_position, |a, b| {
                    match page_ordering_rules.get(a) {
                        Some(followers) => {
                            if followers.contains(b) {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        }
                        None => Ordering::Greater,
                    }
                });
            &*pivot
        })
        .sum())
}

fn parse(input: &str) -> IResult<&str, (Multimap<u32, u32>, Vec<Vec<u32>>)> {
    separated_pair(rules, line_ending, updates)(input)
}

fn rules(input: &str) -> IResult<&str, Multimap<u32, u32>> {
    fold_many1(
        terminated(
            separated_pair(
                character::complete::u32,
                character::complete::char('|'),
                character::complete::u32,
            ),
            line_ending,
        ),
        HashMap::default,
        |mut page_ordering_rules: Multimap<u32, u32>, (page, follower)| {
            page_ordering_rules
                .entry(page)
                .or_insert(HashSet::new())
                .insert(follower);
            page_ordering_rules
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        separated_list1(character::complete::char(','), character::complete::u32),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,75,29,47";

    #[test]
    fn part1_sums_correctly_ordered_updates() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 143);
        Ok(())
    }

    #[test]
    fn part2_sums_incorrectly_ordered_updates() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, 123);
        Ok(())
    }
}
