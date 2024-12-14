use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, claw_machines) = parse(input).map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok(claw_machines.iter().filter_map(optimal_cost).sum())
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, claw_machines) = parse(input).map_err(|e| anyhow!("Unable to parse input: {e}"))?;
    Ok(claw_machines
        .iter()
        .filter_map(corrected_optimal_cost)
        .sum())
}

fn optimal_cost(cm: &ClawMachine) -> Option<i64> {
    let b = (cm.prize.x * cm.a.dy - cm.prize.y * cm.a.dx) / (cm.a.dy * cm.b.dx - cm.a.dx * cm.b.dy);
    let a = (cm.prize.x - cm.b.dx * b) / cm.a.dx;

    if (0..=100).contains(&a)
        && (0..=100).contains(&b)
        && a * cm.a.dx + b * cm.b.dx == cm.prize.x
        && a * cm.a.dy + b * cm.b.dy == cm.prize.y
    {
        Some(3 * a + b)
    } else {
        None
    }
}

fn corrected_optimal_cost(cm: &ClawMachine) -> Option<i64> {
    let prize_x = cm.prize.x + 10_000_000_000_000;
    let prize_y = cm.prize.y + 10_000_000_000_000;
    let b = (prize_x * cm.a.dy - prize_y * cm.a.dx) / (cm.a.dy * cm.b.dx - cm.a.dx * cm.b.dy);
    let a = (prize_x - cm.b.dx * b) / cm.a.dx;

    if a >= 0
        && b >= 0
        && a * cm.a.dx + b * cm.b.dx == prize_x
        && a * cm.a.dy + b * cm.b.dy == prize_y
    {
        Some(3 * a + b)
    } else {
        None
    }
}

#[derive(Debug)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
}

#[derive(Debug)]
struct Button {
    dx: i64,
    dy: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    separated_list1(line_ending, claw_machine)(input)
}

fn claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    map(
        tuple((
            terminated(button_a, line_ending),
            terminated(button_b, line_ending),
            terminated(prize, line_ending),
        )),
        |(button_a, button_b, prize)| ClawMachine {
            a: button_a,
            b: button_b,
            prize,
        },
    )(input)
}

fn button_a(input: &str) -> IResult<&str, Button> {
    map(preceded(tag("Button A: "), deltas), |(dx, dy)| Button {
        dx,
        dy,
    })(input)
}

fn button_b(input: &str) -> IResult<&str, Button> {
    map(preceded(tag("Button B: "), deltas), |(dx, dy)| Button {
        dx,
        dy,
    })(input)
}

fn deltas(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("X+"), character::complete::i64),
        tag(", "),
        preceded(tag("Y+"), character::complete::i64),
    )(input)
}

fn prize(input: &str) -> IResult<&str, Prize> {
    map(
        separated_pair(
            preceded(tag("Prize: X="), character::complete::i64),
            tag(", Y="),
            character::complete::i64,
        ),
        |(x, y)| Prize { x, y },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    pub fn part1_returns_minimum_number_of_tokens() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 480);
        Ok(())
    }
}
