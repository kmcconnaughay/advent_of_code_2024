use advent_of_code_2024::day25::part1;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/day25.txt");
    println!("Day 25 part 1: {}", part1(input)?);
    Ok(())
}
