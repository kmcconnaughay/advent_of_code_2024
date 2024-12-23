use advent_of_code_2024::day24::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day24.txt");

pub fn day24_part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 24 part 1", |b| b.iter(|| part1(black_box(INPUT))));
}

pub fn day24_part2_benchmark(c: &mut Criterion) {
    c.bench_function("day 24 part 2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, day24_part1_benchmark, day24_part2_benchmark);
criterion_main!(benches);