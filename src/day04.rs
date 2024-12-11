const XMAS_DIRECTIONS: [[isize; 2]; 8] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
];

pub fn part1(input: &str) -> u32 {
    let word = "XMAS";
    let num_rows = input.lines().count();
    let num_cols = input
        .lines()
        .next()
        .map(|line| line.len())
        .unwrap_or_default();

    let mut count = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, _letter) in line.char_indices() {
            for [row_direction, col_direction] in XMAS_DIRECTIONS {
                if matches_in_direction(
                    input,
                    word,
                    row as isize,
                    col as isize,
                    row_direction,
                    col_direction,
                    num_rows,
                    num_cols,
                ) {
                    count += 1
                }
            }
        }
    }

    count
}

const X_MAS_DIRECTIONS: [[isize; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];

pub fn part2(input: &str) -> u32 {
    let word = "MAS";
    let num_rows = input.lines().count();
    let num_cols = input
        .lines()
        .next()
        .map(|line| line.len())
        .unwrap_or_default();

    let mut count = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, _letter) in line.char_indices() {
            let mut num_matches = 0;
            for [row_direction, col_direction] in X_MAS_DIRECTIONS {
                if matches_in_direction(
                    input,
                    word,
                    row as isize - row_direction,
                    col as isize - col_direction,
                    row_direction,
                    col_direction,
                    num_rows,
                    num_cols,
                ) {
                    num_matches += 1;
                }
            }

            if num_matches == 2 {
                count += 1;
            }
        }
    }
    count
}

fn matches_in_direction(
    input: &str,
    word: &str,
    row: isize,
    col: isize,
    row_direction: isize,
    col_direction: isize,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    for (index, letter) in word.char_indices() {
        let index = index as isize;
        let next_row = row + index * row_direction;
        let next_col = col + index * col_direction;

        if next_row < 0
            || next_row >= num_rows as isize
            || next_col < 0
            || next_col >= num_cols as isize
        {
            return false;
        }

        let next_row = next_row as usize;
        let next_col = next_col as usize;
        if input.as_bytes()[next_row * (num_cols + 1) + next_col] != letter as u8 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_counts_xmas() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT), 18);
        Ok(())
    }

    #[test]
    fn part2_counts_x_mas() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT), 9);
        Ok(())
    }
}
