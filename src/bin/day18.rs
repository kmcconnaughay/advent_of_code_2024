use advent_of_code_2024::day18::{part1, part2, MAP_DIMS};

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day18.txt");
    println!("Day 18 part 1: {}", part1(input, MAP_DIMS, 1024)?);
    println!("Day 18 part 2: {:?}", part2(input, MAP_DIMS)?);
    Ok(())
}
