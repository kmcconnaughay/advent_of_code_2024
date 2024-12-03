type Report = Vec<i64>;

fn main() {
    let input = include_str!("../data/day2.txt");
    let mut reports: Vec<Report> = vec![];

    for line in input.lines() {
        reports.push(
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i64>().unwrap())
                .collect(),
        );
    }

    println!("Day 2 part 1: {}", part1(&reports));
    println!("Day 2 part 2: {}", part2(&reports));
}

fn part1(reports: &Vec<Report>) -> i64 {
    reports.iter().filter(|report| is_safe(report)).count() as i64
}

fn part2(reports: &Vec<Report>) -> i64 {
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
    use super::{part1, part2, Report};

    #[test]
    fn part1_returns_num_safe_levels() {
        let reports: Vec<Report> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let num_safe_levels = part1(&reports);

        assert_eq!(num_safe_levels, 2);
    }

    #[test]
    fn part2_returns_num_safe_levels_with_dampener() {
        let reports: Vec<Report> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let num_safe_levels = part2(&reports);

        assert_eq!(num_safe_levels, 4);
    }
}
