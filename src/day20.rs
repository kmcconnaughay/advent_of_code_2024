pub fn part1(_input: &str) -> anyhow::Result<u32> {
    todo!()
}

pub fn part2(_input: &str) -> anyhow::Result<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_returns_number_of_cheats_that_reduce_cost_by_100() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 44);
        Ok(())
    }
}
