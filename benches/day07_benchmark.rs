use advent_of_code_2024::day07::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day07.txt");

pub fn day07_part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 7 part 1", |b| b.iter(|| part1(black_box(INPUT))));
}

pub fn day07_part2_benchmark(c: &mut Criterion) {
    c.bench_function("day 7 part 2", |b| b.iter(|| part2(black_box(INPUT))));
}

criterion_group!(benches, day07_part1_benchmark, day07_part2_benchmark);
criterion_main!(benches);
