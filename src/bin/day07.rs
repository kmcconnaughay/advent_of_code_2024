use advent_of_code_2024::day07::{part1, part2};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day07.txt");
    println!("Day 07 part 1: {}", part1(input)?);
    println!("Day 07 part 2: {}", part2(input)?);

    Ok(())
}
