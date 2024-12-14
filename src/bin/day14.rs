use advent_of_code_2024::day14::{part1, part2, MAP_DIMS};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day14.txt");
    println!("Day 14 part 1: {}", part1(input, MAP_DIMS)?);
    println!("Day 14 part 2: {}", part2(input, MAP_DIMS)?);
    Ok(())
}
