use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

type Multimap<K, V> = HashMap<K, Vec<V>>;

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let (raw_page_ordering_rules, raw_updates) = input.split_once("\n\n").ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unable to split input",
    ))?;

    let page_ordering_rules = parse_page_ordering_rules(raw_page_ordering_rules)?;

    Ok(raw_updates
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .filter(|update| {
            update.windows(2).all(|window| {
                page_ordering_rules
                    .get(window[0])
                    .map_or(false, |followers| followers.contains(&window[1]))
            })
        })
        .filter_map(|update| update[update.len() / 2].parse::<u64>().ok())
        .sum())
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let (raw_page_ordering_rules, raw_updates) = input.split_once("\n\n").ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unable to split input",
    ))?;

    let page_ordering_rules = parse_page_ordering_rules(raw_page_ordering_rules)?;

    Ok(raw_updates
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .filter(|update| {
            !update.windows(2).all(|window| {
                page_ordering_rules
                    .get(window[0])
                    .map_or(false, |followers| followers.contains(&window[1]))
            })
        })
        .filter_map(|mut update| {
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
            pivot.parse::<u64>().ok()
        })
        .sum())
}

fn parse_page_ordering_rules(
    raw_page_ordering_rules: &str,
) -> anyhow::Result<Multimap<&str, &str>> {
    let mut page_ordering_rules = HashMap::<&str, Vec<&str>>::new();
    for rule in raw_page_ordering_rules.lines() {
        let (page, follower) = rule.split_once('|').ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unable to parse page ordering rule: {}", rule),
        ))?;
        page_ordering_rules
            .entry(page)
            .or_insert(Vec::new())
            .push(follower);
    }
    Ok(page_ordering_rules)
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
