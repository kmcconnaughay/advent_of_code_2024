use advent_of_code_2024::day25::part1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day25.txt");

pub fn day25_part1_benchmark(c: &mut Criterion) {
    c.bench_function("day 25 part 1", |b| b.iter(|| part1(black_box(INPUT))));
}

criterion_group!(benches, day25_part1_benchmark);
criterion_main!(benches);
