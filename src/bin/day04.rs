use advent_of_code_2024::day04::{part1, part2};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day04.txt");
    println!("Day 4 part 1: {}", part1(input));
    println!("Day 4 part 1: {}", part2(input));

    Ok(())
}
