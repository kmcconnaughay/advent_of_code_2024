use advent_of_code_2024::day20::{part1, part2};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day20.txt");
    println!("Day 20 part 1: {}", part1(input)?);
    println!("Day 20 part 2: {:?}", part2(input)?);
    Ok(())
}
