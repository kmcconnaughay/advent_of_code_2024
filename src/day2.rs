type Report = Vec<i64>;

pub fn part1(input: &str) -> i64 {
    let mut reports: Vec<Report> = vec![];

    for line in input.lines() {
        reports.push(
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect(),
        );
    }

    reports.iter().filter(|report| is_safe(report)).count() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut reports: Vec<Report> = vec![];

    for line in input.lines() {
        reports.push(
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect(),
        );
    }

    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as i64
}

fn is_safe(report: &Report) -> bool {
    let decreasing = report[0] > report[1];

    for pair in report.windows(2) {
        let difference = pair[0] - pair[1];
        let magnitude = difference.abs();

        if decreasing != (difference > 0) {
            return false;
        }

        if magnitude < 1 || magnitude > 3 {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    for (index, _) in report.iter().enumerate() {
        let mut dampened = report.clone();
        dampened.remove(index);
        if is_safe(&dampened) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_returns_num_safe_levels() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn part2_returns_num_safe_levels_with_dampener() {
        assert_eq!(part2(INPUT), 4);
    }
}
