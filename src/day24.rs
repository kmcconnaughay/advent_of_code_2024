use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{self, separated_pair},
    IResult,
};

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let circuit = parse(input)?;

    let mut known = circuit
        .wires
        .iter()
        .map(|wire| (wire.label.as_str(), wire.initial_value))
        .collect::<HashMap<&str, bool>>();

    while known.len() < circuit.num_wires() {
        for gate in &circuit.gates {
            if known.contains_key(gate.output.as_str()) {
                continue;
            }

            if let Some(left) = known.get(gate.left.as_str()) {
                if let Some(right) = known.get(gate.right.as_str()) {
                    known.insert(&gate.output, gate.operation.eval(*left, *right));
                }
            }
        }
    }

    let mut output = 0;
    for wire in circuit.output_wires() {
        output <<= 1;
        if known[wire] {
            output |= 1;
        }
    }

    Ok(output)
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let circuit = parse(input)?;

    let highest_z = circuit
        .output_wires()
        .last()
        .ok_or_else(|| anyhow!("No output wires found"))?;

    let wrong = circuit
        .gates
        .iter()
        .filter_map(|gate| {
            if Classification::of_wire(&gate.output) == Classification::Output
                && gate.operation != Operation::Xor
                && gate.output != highest_z
            {
                return Some(gate.output.as_str());
            }

            if gate.operation == Operation::And
                && gate.left != "x00"
                && gate.right != "x00"
                && circuit.gates.iter().any(|sub_op| {
                    sub_op.operation != Operation::Or
                        && (gate.output == sub_op.left || gate.output == sub_op.right)
                })
            {
                return Some(gate.output.as_str());
            }

            if gate.operation == Operation::Xor {
                if Classification::of_wire(&gate.left) == Classification::Other
                    && Classification::of_wire(&gate.right) == Classification::Other
                    && Classification::of_wire(&gate.output) == Classification::Other
                {
                    return Some(gate.output.as_str());
                }

                if circuit.gates.iter().any(|sub_op| {
                    sub_op.operation == Operation::Or
                        && (gate.output == sub_op.left || gate.output == sub_op.right)
                }) {
                    return Some(gate.output.as_str());
                }
            }

            None
        })
        .sorted()
        .join(",");

    Ok(wrong)
}

struct Circuit {
    wires: Vec<Wire>,
    gates: Vec<Gate>,
}

impl Circuit {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                separated_list1(line_ending, Wire::parse),
                many1(line_ending),
                separated_list1(line_ending, Gate::parse),
            ),
            |(wires, gates)| Circuit { wires, gates },
        )(input)
    }

    fn num_wires(&self) -> usize {
        self.gates
            .iter()
            .flat_map(|gate| [&gate.left, &gate.right, &gate.output].into_iter())
            .unique()
            .count()
    }

    fn output_wires(&self) -> impl Iterator<Item = &str> {
        self.gates
            .iter()
            .filter_map(|gate| {
                if gate.output.starts_with("z") {
                    Some(gate.output.as_str())
                } else {
                    None
                }
            })
            .sorted()
    }
}

struct Wire {
    label: String,
    initial_value: bool,
}

impl Wire {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                alphanumeric1,
                tag(": "),
                alt((
                    value(false, complete::char('0')),
                    value(true, complete::char('1')),
                )),
            ),
            |(label, initial_value): (&str, bool)| Wire {
                label: label.to_string(),
                initial_value,
            },
        )(input)
    }
}

struct Gate {
    left: String,
    right: String,
    output: String,
    operation: Operation,
}

impl Gate {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            sequence::tuple((
                alphanumeric1,
                complete::char(' '),
                Operation::parse,
                complete::char(' '),
                alphanumeric1,
                tag(" -> "),
                alphanumeric1,
            )),
            |(left, _, operation, _, right, _, output)| Gate {
                left: left.to_string(),
                right: right.to_string(),
                output: output.to_string(),
                operation,
            },
        )(input)
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Operation {
    And,
    Xor,
    Or,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Operation::And, tag("AND")),
            value(Operation::Xor, tag("XOR")),
            value(Operation::Or, tag("OR")),
        ))(input)
    }

    fn eval(&self, left: bool, right: bool) -> bool {
        match self {
            Operation::And => left && right,
            Operation::Xor => left != right,
            Operation::Or => left || right,
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Circuit> {
    let (_, circuit) = all_consuming(Circuit::parse)(input)
        .map_err(|e| anyhow!("Unable to parse input: {}", e))?;
    Ok(circuit)
}

#[derive(PartialEq, Eq)]
enum Classification {
    Input,
    Output,
    Other,
}

impl Classification {
    fn of_wire(wire: &str) -> Self {
        match wire.chars().next() {
            Some('x') | Some('y') => Classification::Input,
            Some('z') => Classification::Output,
            _ => Classification::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_simulates_circuit() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, 2024);
        Ok(())
    }

    #[test]
    fn part2_identifies_swapped_wires() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, "aaa,aoc,bbb,ccc,eee,ooo,z24,z99");
        Ok(())
    }
}
