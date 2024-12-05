use advent_of_code_2024::day2::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day2.txt");

pub fn part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 2 part 1", |b| b.iter(|| part1(black_box(INPUT))));
}

pub fn part2_benchmark(c: &mut Criterion) {
    c.bench_function("day 2 part 2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);